use std::mem;
use crate::app::App;
use crate::component::Component;
use crate::macros::collides;
use crate::rectangle::Rectangle;
use crate::text::Text;
use crate::text_input_handler::{IdxSize, TextInputHandler};

pub struct Textbox {
    handler: TextInputHandler,
    position: (i32, i32),
    size: (u32, u32),
    selected: bool,
    hovered: bool,
    text: Text,
    pub children: Vec<Box<dyn Component>>,
    background_object: Option<Box<dyn Component>>,
    z_index: f32
}


impl Textbox {
    pub fn new(position: (i32, i32), size: (u32, u32), content: &str, allow_newlines: bool, max_length: Option<IdxSize>, allow_editing: bool, z_index: f32, color: (u8, u8, u8, u8)) -> Self {

        Self {
            handler: TextInputHandler::new(content.to_string(), allow_newlines, max_length, allow_editing),
            position,
            size,
            selected: false,
            hovered: false,
            text: Text::new(0, 0, "", (Some(position.0), Some(position.1), Some(size.0), Some(size.1)), 16.0/50.0, z_index, color),
            children: Vec::new(),
            background_object: None,
            z_index
        }
    }
    
    pub fn set_bg_color(&mut self, color: (u8, u8, u8, u8)) {
        
        self.background_object = Some(
            Box::new(
                Rectangle::new(self.position.0, self.position.1, self.size.0, self.size.1, color, self.z_index)
            )
        );
    }
    
    pub fn remove_background(&mut self) {
        self.background_object = None;
    }
    
}


impl Component for Textbox {
    fn update(&mut self, app: &mut App) {
        self.hovered = collides!(app, self, app.mouse.position);

        if app.mouse.left_down {
            self.selected = self.hovered;
        }

        if self.selected {
            self.handler.process(app);
        }

        self.text.content = self.handler.content.to_string();

        app.camera.push();

        app.camera.set_ipos(self.position.0, self.position.1);

        if let Some(bg) = &mut self.background_object {
            bg.update(app);
        }
        
        for child in &mut self.children {
            child.update(app);
        }
        
        self.text.update(app);

        app.camera.pop();

    }

    fn destroy(self) {
    }
}


