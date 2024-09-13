use sdl2::event::Event;

use crate::{camera::Camera, component::Component, image::Image, rectangle::Rectangle, shaders::Shaders, text::CharAtlas, texture_atlas::{convert_tex_to_gl, TextureAtlas}, window_frame::WindowFrame};

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
    pub scroll_y: i32
}

impl Mouse {
    pub fn new() -> Self {
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
            scroll_y: 0
        }
    }
}

pub struct App<'a> {
    pub tex_atlas: TextureAtlas<'a>,
    pub events: Vec<Event>,
    pub shaders: Shaders,
    pub char_atlas: CharAtlas,
    pub window_size: (u32, u32),
    pub camera: Camera,
    children: Vec<Box<dyn Component>>,

    pub mouse: Mouse
}


impl<'a> App<'a> {
    pub fn new(shaders: Shaders, char_atlas: CharAtlas, window_width: u32, window_height: u32) -> App<'a> {

        let mut tex_atlas = TextureAtlas::new();

        let i: u32 = 0;
        for tex in & tex_atlas.textures {
            let (val, (_, _)) = convert_tex_to_gl(tex, 0);
            tex_atlas.idx_to_gluint.insert(i, val);
        }


        App {
            tex_atlas,
            events: Vec::new(),
            shaders,
            char_atlas,
            window_size: (0, 0),
            camera: Camera::new(window_width, window_height),
            children: vec![
                Box::new(Rectangle::new(10, 10, 100, 100, (255, 127, 0, 255), 0.0)),
                Box::new(Image::new(500, 100, 100, 100, "assets/textures/idnh_logo.png".to_string(), (0, 0, 128, 128), 0.1)),
                Box::new(Image::new(120, 10, 1000, 1000, "assets/textures/idnh_logo.png".to_string(), (0, 0, 128, 128), 0.0)),
                Box::new(WindowFrame::new(
                    "Insert Dungeon Name Here".to_string(),
                    Image::new(0, 0, 24, 24, "assets/textures/idnh_icon.png".to_string(), (0, 0, 32, 32), 0.91),
                    (window_width, window_height)
                )),

            ],
            mouse: Mouse::new()
        }
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn update(&mut self) {

        let mut children = std::mem::take(&mut self.children);

        for child in &mut children {
            child.update(self);
        }

        self.children = children;

    }

    pub fn map_coords(&self, pos: &(i32, i32)) -> (f32, f32) {
        ((pos.0 as f32 * 2.0 / self.window_size.1 as f32) - (self.window_size.0 as f32 / self.window_size.1 as f32), 1.0 - (pos.1 as f32 * 2.0 / self.window_size.1 as f32))
    }

    pub fn map_size(&self, size: &(u32, u32)) -> (f32, f32) {
        (size.0 as f32 / self.window_size.1 as f32, size.1 as f32 / self.window_size.1 as f32)
    }

}
