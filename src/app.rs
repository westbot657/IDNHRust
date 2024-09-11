use sdl2::event::Event;

use crate::{component::Component, image::Image, rectangle::Rectangle, shaders::Shaders, texture_atlas::{convert_tex_to_gl, TextureAtlas}};



pub struct App<'a> {
    pub tex_atlas: TextureAtlas<'a>,
    pub events: Vec<Event>,
    pub shaders: Shaders,
    pub window_size: (u32, u32),
    children: Vec<Box<dyn Component>>
}


impl<'a> App<'a> {
    pub fn new(shaders: Shaders) -> App<'a> {

        let mut tex_atlas = TextureAtlas::new();

        let i: u32 = 0;
        for tex in & tex_atlas.textures {
            let (val, (_, _)) = convert_tex_to_gl(tex);
            tex_atlas.idx_to_gluint.insert(i, val);
        }

        App {
            tex_atlas,
            events: Vec::new(),
            shaders,
            window_size: (0, 0),
            children: vec![

                Box::new(Rectangle::new(10, 10, 100, 100, (255, 127, 0, 255))),
                Box::new(Image::new(500, 100, 100, 100, "assets\\textures\\test2.png".to_string(), (0, 0, 128, 128), (128, 128), -0.1)),
                Box::new(Image::new(120, 10, 1000, 1000, "assets\\textures\\test2.png".to_string(), (0, 0, 128, 128), (128, 128), 0.0)),

            ]
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

    pub fn mapCoords(&self, pos: &(i32, i32)) -> (f32, f32) {
        ((pos.0 as f32 / self.window_size.0 as f32) - 1.0, 1.0 - (pos.1 as f32 / self.window_size.1 as f32))
    }

    pub fn mapSize(&self, size: &(u32, u32)) -> (f32, f32) {
        (size.0 as f32 / self.window_size.0 as f32, size.1 as f32 / self.window_size.1 as f32)
    }

}