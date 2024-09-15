use std::{collections::HashMap, time};

use cgmath::{Matrix4, SquareMatrix, Vector4};
use enigo::{Enigo, Settings};
use sdl2::{event::Event, video::Window};

use crate::{app_selector::AppSelector, camera::Camera, component::Component, image::Image, macros::{cast_component, SETTINGS}, shaders::Shaders, text::{CharAtlas, Text}, texture_atlas::{convert_tex_to_gl, TextureAtlas}, window_frame::WindowFrame};

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

    pub cursor_style: Option<String>,
    pub cursors: HashMap<String, sdl2::mouse::Cursor>,
    pub active_cursor_style: Option<String>,
    pub previous_cursor_style: Option<String>
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

            cursor_style: None,
            cursors,
            active_cursor_style: None,
            previous_cursor_style: None
        }
    }
}

pub struct App<'a> {
    pub tex_atlas: TextureAtlas<'a>,
    pub events: Vec<Event>,
    pub shaders: Shaders,
    pub char_atlas: CharAtlas,
    pub window_pos: (i32, i32),
    pub window_size: (u32, u32),
    pub camera: Camera,
    children: Vec<Box<dyn Component>>,

    pub mouse: Mouse,
    pub enigo: Enigo,
    pub window: &'a mut Window,
    pub should_quit: bool,
    pub fullscreen: bool,
    pub pre_fullscreen_pos: (i32, i32),
    pub pre_fullscreen_size: (u32, u32),
}


impl<'a> App<'a> {
    pub fn new(shaders: Shaders, char_atlas: CharAtlas, window_width: u32, window_height: u32, window: &'a mut Window) -> App<'a> {

        let mut tex_atlas = TextureAtlas::new();

        let i: u32 = 0;
        for tex in & tex_atlas.textures {
            let (val, (_, _)) = convert_tex_to_gl(tex, 0);
            tex_atlas.idx_to_gluint.insert(i, val);
        }


        let mut app = App {
            tex_atlas,
            events: Vec::new(),
            shaders,
            char_atlas,
            window_pos: (0, 0),
            window_size: (0, 0),
            camera: Camera::new(window_width, window_height),
            children: Vec::new(),
            mouse: Mouse::new(),
            enigo: Enigo::new(&Settings::default()).unwrap(),
            window,
            should_quit: false,
            fullscreen: false,
            pre_fullscreen_pos: (0, 0),
            pre_fullscreen_size: (0, 0)
        };

        let app_selector = AppSelector::new(&app);

        app.children = vec![
            Box::new(WindowFrame::new(
                "Insert Dungeon Name Here".to_string(),
                Image::new(3, 3, 18, 18, "assets/textures/idnh_icon.png".to_string(), (0, 0, 36, 36), 0.91),
                (window_width, window_height),
                &app
            )),

            Box::new(Text::new(0, 0, "FPS".to_string(), None, 0.3, 1.0, SETTINGS!(text color 4 u8))),


            Box::new(app_selector)

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

        let mut children = std::mem::take(&mut self.children);

        children[0].update(self);

        self.camera.push();

        self.camera.set_position(5.0 / self.window_size.0 as f32, 25.0 / self.window_size.1 as f32);
        self.camera.viewport = (5, 25, self.window_size.0-5, self.window_size.1-20);

        for child in &mut children[2..] {
            child.update(self);
        }

        self.camera.pop();

        if self.mouse.active_cursor_style.is_some() {
            self.mouse.cursors.get(self.mouse.active_cursor_style.as_ref().unwrap()).unwrap().set();
        } else {
            self.mouse.cursors.get("Arrow").unwrap().set();
        }
        
        let fps = dt.elapsed().as_secs_f64();
        
        let fps_counter = cast_component!(children.get_mut(1).unwrap() => mut Text);
        fps_counter.content = format!("{}", 1.0/fps);
        fps_counter.content = format!("FPS: {}", fps_counter.content[0..8.min(fps_counter.content.len()-1)].to_string());
        fps_counter.position = (5, (self.window_size.1 - 20) as i32);
        
        fps_counter.update(self);
        
        self.children = children;

    }

    pub fn map_coords(&self, pos: &(i32, i32)) -> (f32, f32) {
        ((pos.0 as f32 * 2.0 / self.window_size.1 as f32) - (self.window_size.0 as f32 / self.window_size.1 as f32), 1.0 - (pos.1 as f32 * 2.0 / self.window_size.1 as f32))
    }

    pub fn map_size(&self, size: &(u32, u32)) -> (f32, f32) {
        (size.0 as f32 / self.window_size.1 as f32, size.1 as f32 / self.window_size.1 as f32)
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
    
        let (camera_matrix, viewport) = self.camera.peek();
        let combined_matrix = camera_matrix * transform_matrix;

        if !(viewport.0 <= point.0 && point.0 <= viewport.0 + viewport.2 as i32 &&
            viewport.1 <= point.1 && point.1 <= viewport.1 + viewport.3 as i32) {
                return false
            }
    
        let screen_coords = Vector4::new(
            2.0 * (point.0 as f32 / self.window_size.0 as f32) - 1.0,
            1.0 - 2.0 * (point.1 as f32 / self.window_size.1 as f32),
            0.0,
            1.0,
        );
    
        if let Some(inv_combined_matrix) = combined_matrix.invert() {
            let transformed_point = inv_combined_matrix * screen_coords;
            let world_x = transformed_point.x * 2.0;
            let world_y = transformed_point.y * 2.0;
    
            world_x >= -1.0 && world_x <= 1.0 && world_y >= -1.0 && world_y <= 1.0
        } else {
            false
        }
    }

    /// Checks for a plain collision without accounting for camera transformations
    pub fn raw_collides(rect: (i32, i32, u32, u32), point: (i32, i32)) -> bool {
        rect.0 <= point.0 && point.0 <= rect.0 + rect.2 as i32 &&
        rect.1 <= point.1 && point.1 <= rect.1 + rect.3 as i32

    }

}
