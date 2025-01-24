use std::{collections::HashMap, mem, time};
use cgmath::{Matrix4, SquareMatrix, Vector4};
use enigo::{Enigo, Mouse as eMouse, Settings};
use sdl2::{event::Event, video::Window};

use crate::{app_selector::AppSelector, camera::Camera, component::Component, image::Image, keybinds::Keybinds, macros::{cast_component, SETTINGS}, shaders::Shaders, text::Text, texture_atlas::{convert_tex_to_gl, TextureAtlas}, window_frame::WindowFrame};
use crate::component_system::{ComponentSystem, SystematicComponent};
use crate::history_manager::HistoryManager;
use crate::macros::font_size;
use crate::nine_slice::NineSliceBase;
use crate::text::FontHandler;
use crate::toast_system::ToastSystem;

pub struct Mouse {
    pub left_down: bool,
    pub left_up: bool,
    pub left_held: bool,

    pub middle_down: bool,
    pub middle_up: bool,
    pub middle_held: bool,

    pub right_down: bool,
    pub right_up: bool,
    pub right_held: bool,

    pub scroll_x: i32,
    pub scroll_y: i32,

    pub position: (i32, i32),

    pub cursors: HashMap<String, sdl2::mouse::Cursor>,
    pub active_cursor_style: Option<String>,
    pub held_object: Option<Box<dyn Component>>,
    pub hold_offset: (i32, i32),
    pub hold_scale: f32
}

impl Mouse {
    pub fn new() -> Self {

        let mut cursors = HashMap::new();

        cursors.insert("SizeWE".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeWE).unwrap());
        cursors.insert("SizeNS".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNS).unwrap());
        cursors.insert("SizeNWSE".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNWSE).unwrap());
        cursors.insert("SizeNESW".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNESW).unwrap());
        cursors.insert("SizeAll".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeAll).unwrap());
        cursors.insert("Hand".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Hand).unwrap());
        cursors.insert("Arrow".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Arrow).unwrap());
        cursors.insert("IBeam".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::IBeam).unwrap());
        cursors.insert("Crosshair".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Crosshair).unwrap());
        cursors.insert("No".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::No).unwrap());
        cursors.insert("Wait".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Wait).unwrap());
        cursors.insert("WaitArrow".to_string(), sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::WaitArrow).unwrap());


        Self {
            left_down: false,
            left_up: false,
            left_held: false,
            middle_down: false,
            middle_up: false,
            middle_held: false,
            right_down: false,
            right_up: false,
            right_held: false,
            scroll_x: 0,
            scroll_y: 0,
            position: (0, 0),

            cursors,
            active_cursor_style: None,
            held_object: None,
            hold_offset: (0, 0),
            hold_scale: 1.0
        }
    }
    
    pub fn copy_state(&mut self, other: &Mouse) {
        self.left_down = other.left_down;
        self.left_up = other.left_up;
        self.left_held = other.left_held;
        
        self.middle_down = other.middle_down;
        self.middle_up = other.middle_up;
        self.middle_held = other.middle_held;
        
        self.right_down = other.right_down;
        self.right_up = other.right_up;
        self.right_held = other.right_held;
        
        self.scroll_x = other.scroll_x;
        self.scroll_y = other.scroll_y;
        self.position = other.position;
    }
    
}

pub struct Keyboard {
    pub shift_held: bool,
    pub lshift_held: bool,
    pub rshift_held: bool,

    pub ctrl_held: bool,
    pub lctrl_held: bool,
    pub rctrl_held: bool,

    pub alt_held: bool,
    pub lalt_held: bool,
    pub ralt_held: bool,
    pub capslock: bool,

    pub held_keys: Vec<String>,
    pub newly_pressed_keys: Vec<String>,
    pub released_keys: Vec<String>,
    pub triggered_keys: Vec<String>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            shift_held: false,
            lshift_held: false,
            rshift_held: false,
            ctrl_held: false,
            lctrl_held: false,
            rctrl_held: false,
            alt_held: false,
            lalt_held: false,
            ralt_held: false,
            capslock: false,
            held_keys: Vec::new(),
            newly_pressed_keys: Vec::new(),
            released_keys: Vec::new(),
            triggered_keys: Vec::new()
        }
    }
}

pub struct App<'a> {
    pub tex_atlas: TextureAtlas<'a>,
    pub events: Vec<Event>,
    pub shaders: Shaders,
    pub font_handler: FontHandler,
    pub window_pos: (i32, i32),
    pub window_size: (u32, u32),
    pub camera: Camera,
    children: Vec<Box<dyn Component>>,

    pub mouse: Mouse,
    pub last_mouse: Mouse,
    pub keyboard: Keyboard,
    pub enigo: Enigo,
    pub window: &'a mut Window,
    pub should_quit: bool,
    pub fullscreen: bool,
    pub pre_fullscreen_pos: (i32, i32),
    pub pre_fullscreen_size: (u32, u32),
    pub monitors: Vec<(i32, i32, u32, u32)>,
    pub keybinds: Keybinds,
    pub settings: crate::settings::Settings,
    pub history: HistoryManager,

    pub uid: String,

    path: Vec<String>,

    toasts: ToastSystem,
    _toasts: Option<ToastSystem>,
    
    pub component_system: ComponentSystem,
    
    pub nine_slices: HashMap<String, NineSliceBase>

}


impl<'a> App<'a> {
    pub fn new(shaders: Shaders, font_handler: FontHandler, window_width: u32, window_height: u32, window: &'a mut Window, monitors: Vec<(i32, i32, u32, u32)>) -> App<'a> {

        let mut tex_atlas = TextureAtlas::new();

        let i: u32 = 0;
        for tex in & tex_atlas.textures {
            let (val, (_, _)) = convert_tex_to_gl(tex, 0);
            tex_atlas.idx_to_gluint.insert(i, val);
        }

        let mut settings = crate::settings::Settings::new();

        settings.load();
        settings.save();

        let mut app = App {
            tex_atlas,
            events: Vec::new(),
            shaders,
            font_handler,
            window_pos: (0, 0),
            window_size: (0, 0),
            camera: Camera::new(),
            children: Vec::new(),
            mouse: Mouse::new(),
            last_mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            enigo: Enigo::new(&Settings::default()).unwrap(),
            window,
            should_quit: false,
            fullscreen: false,
            pre_fullscreen_pos: (0, 0),
            pre_fullscreen_size: (0, 0),
            monitors,
            keybinds: Keybinds::new(&settings),
            settings,
            history: HistoryManager::new(),
            uid: "App".to_string(),
            path: Vec::new(),
            toasts: ToastSystem::blank(),
            _toasts: None,
            
            component_system: ComponentSystem::new(),
            nine_slices: HashMap::new(),
        };
        let mut tsts = ToastSystem::new(&app, 300, 80);
        mem::swap(&mut tsts, &mut app.toasts);
        app._toasts = Some(tsts);

        let app_selector = AppSelector::new(&mut app);

        app.children = vec![
            Box::new(WindowFrame::new(
                "Insert Dungeon Name Here",
                Image::new(3, 3, 18, 18, "assets/textures/idnh_icon.png".to_string(), (0, 0, 36, 36), 0.91),
                (window_width, window_height),
                &app
            )),

            Box::new(Text::new(0, 0, "FPS", (None, None, None, None), font_size!(15.0), 1.0, SETTINGS!(text color 4 u8))),


            Box::new(app_selector.systemize(&mut app.component_system))

        ];


        app
    }
    
    pub fn set_cursor(&mut self, cursor: String) {
        if self.mouse.cursors.contains_key(&cursor) {
            self.mouse.active_cursor_style = Some(cursor);

        }
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn update(&mut self) {
        
        let dt = time::Instant::now();
        self.mouse.active_cursor_style = None;

        let mut children = mem::take(&mut self.children);
        self.camera.push();
        self.camera.aspect_ratio = self.window_size.1 as f32 / self.window_size.0 as f32;
        self.camera.window_height = self.window_size.1;
        // self.camera.set_ipos(5, 25);
        self.camera.scale(self.camera.aspect_ratio, 1.0, 1.0);

        children[0].update(self);


        let pos = self.map_coords(&(5, -25));
        self.camera.translate(pos.0 - self.camera.aspect_ratio, 1.0 - pos.1, 0f32);

        // self.camera.set_position(5.0 / self.window_size.0 as f32, 25.0 / self.window_size.1 as f32);
        self.camera.set_viewport((5, 25, self.window_size.0-5, self.window_size.1-25));

        for child in &mut children[2..] {
            child.update(self);
        }
        let mut tst = mem::take(&mut self._toasts).unwrap();
        mem::swap(&mut tst, &mut self.toasts);

        tst.update(self);

        mem::swap(&mut tst, &mut self.toasts);
        self._toasts = Some(tst);

        self.camera.pop();

        if self.mouse.active_cursor_style.is_some() {
            self.mouse.cursors.get(self.mouse.active_cursor_style.as_ref().unwrap()).unwrap().set();
        } else {
            self.mouse.cursors.get("Arrow").unwrap().set();
        }

        if self.keybinds.check_binding("Save") {
            self.keybinds.accept(&vec!["S"]);
            self.toasts.push("Testing a longer message\nwrapping has to be done\nmanually", Vec::new(), None, None);
        }

        let fps = dt.elapsed().as_secs_f64();
        
        let fps_counter = cast_component!(children.get_mut(1).unwrap() => mut Text);
        let mut c = format!("{}", 1.0/fps);
        c = format!("FPS: {}",
            &c[0..8.min(c.len()-1)],
        );
        fps_counter.content = c;
        fps_counter.position = (5, (self.window_size.1 - 20) as i32);
        
        fps_counter.update(self);
        
        self.children = children;

    }

    pub fn map_coords(&self, pos: &(i32, i32)) -> (f32, f32) {
        (
            (pos.0 as f32 * 2.0 / self.window_size.1 as f32) - 1.0,
            1.0 - (pos.1 as f32 * 2.0 / self.window_size.1 as f32)
        )
    }

    pub fn unmap_coords(&self, pos: &(f32, f32)) -> (i32, i32) {
        (
            (((pos.0 + 1.0) * self.window_size.1 as f32) / 2.0).round() as i32,
            (((1.0 - pos.1) * self.window_size.1 as f32) / 2.0).round() as i32
        )
    }

    pub fn map_size(&self, size: &(u32, u32)) -> (f32, f32) {
        (
            size.0 as f32 / self.window_size.1 as f32,
            size.1 as f32 / self.window_size.1 as f32
        )
    }
    
    pub fn unmap_size(&self, size: &(f32, f32)) -> (u32, u32) {
        (
            (size.0 * self.window_size.1 as f32) as u32,
            (size.1 * self.window_size.1 as f32) as u32
        )
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.window_pos = (x, y);
        self.window.set_position(sdl2::video::WindowPos::Positioned(x), sdl2::video::WindowPos::Positioned(y))
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        self.window_size = (size.0, size.1);
        self.window.set_size(size.0, size.1).unwrap();
    }

    /// check for a collision, including any transformations made to the camera
    pub fn collides(&self, rect: (i32, i32, u32, u32), point: (i32, i32)) -> bool {
        let pos = self.map_coords(&(rect.0, rect.1));
        let sz = self.map_size(&(rect.2, rect.3));
    
        let transform_matrix = Matrix4::from_cols(
            Vector4::new(sz.0 * 2.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, sz.1 * 2.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(pos.0 + (sz.0), pos.1 - (sz.1), 0.0, 1.0),
        );
    
        let (translation_matrix, normal_matrix, _, viewport) = self.camera.peek();
        let combined_matrix = translation_matrix * transform_matrix;

        if !(viewport.0 <= point.0 && point.0 <= viewport.0 + viewport.2 as i32 &&
            viewport.1 <= point.1 && point.1 <= viewport.1 + viewport.3 as i32) {
                return false
            }
        
        if let Some(inv_combined_matrix) = combined_matrix.invert() {
            let screen_coords = Vector4::new(
                2.0 * (point.0 as f32 / self.window_size.0 as f32) - 1.0,
                1.0 - 2.0 * (point.1 as f32 / self.window_size.1 as f32),
                0.0,
                1.0,
            );
            let transformed_point = inv_combined_matrix * screen_coords;
            let world_x = transformed_point.x * 2.0;
            let world_y = transformed_point.y * 2.0;

            (-1.0..1.0).contains(&world_x) && (-1.0..1.0).contains(&world_y)
        } else {
            false
        }
    }

    pub fn map_rect(&self, rect: (i32, i32, u32, u32)) -> (i32, i32, u32, u32) {
        let pos = self.map_coords(&(rect.0, rect.1));
        let sz = self.map_size(&(rect.2, rect.3));

        let transform_matrix = Matrix4::from_cols(
            Vector4::new(sz.0 * 2.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, sz.1 * 2.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(pos.0 + sz.0, pos.1 - sz.1, 0.0, 1.0),
        );

        let (translation_matrix, normal_matrix, _, viewport) = self.camera.peek();
        let combined_matrix: Matrix4<f32> = translation_matrix * transform_matrix;

        let mut transformed_corners = Vec::new();

        let corners = [
            Vector4::new(-1.0, -1.0, 0.0, 1.0), // Top-left
            Vector4::new(1.0, -1.0, 0.0, 1.0),  // Top-right
            Vector4::new(-1.0, 1.0, 0.0, 1.0),  // Bottom-left
            Vector4::new(1.0, 1.0, 0.0, 1.0),   // Bottom-right
        ];

        for corner in corners.iter() {
            let transformed_corner: Vector4<f32> = combined_matrix * *corner;
            let (mx, my) = self.unmap_coords(&(transformed_corner.x, transformed_corner.y));
            transformed_corners.push((
                mx, my
            ));
        }

        let min_x = transformed_corners.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let max_x = transformed_corners.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let min_y = transformed_corners.iter().map(|(_, y)| *y).min().unwrap_or(0);
        let max_y = transformed_corners.iter().map(|(_, y)| *y).max().unwrap_or(0);

        let width = (max_x - min_x).abs() as u32;
        let height = (max_y - min_y).abs() as u32;

        println!("output rect: {}, {}, {}, {}", min_x, min_y, width, height);
        (min_x, min_y, width, height)
    }


    /// In theory this function should only ever return Some()
    pub fn get_monitor_with_cursor(&self) -> Option<(i32, i32, u32, u32)> {

        let (mx, my) = self.enigo.location().unwrap();
        for monitor in &self.monitors {
            if monitor.0 <= mx && mx <= monitor.0 + monitor.2 as i32 &&
                monitor.1 <= my && my <= monitor.1 + monitor.3 as i32 {
                return Some(*monitor)
            }
        }

        None

    }

}
