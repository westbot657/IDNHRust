use std::collections::VecDeque;
use std::time::Instant;
use crate::app::App;
use crate::component::Component;
use crate::macros::{collides, font_size};
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
    z_index: f32,
    cursor_blink_delta: Instant,
    cursor_rectangle: Rectangle,
    pub uid: String
}


impl Textbox {
    pub fn new(position: (i32, i32), size: (u32, u32), content: &str, allow_newlines: bool, max_length: Option<IdxSize>, allow_editing: bool, z_index: f32, color: (u8, u8, u8, u8)) -> Self {

        Self {
            handler: TextInputHandler::new(content.to_string(), allow_newlines, max_length, allow_editing),
            position,
            size,
            selected: false,
            hovered: false,
            text: Text::new(0, 0, "", (Some(0), Some(0), Some(size.0 - 5), Some(size.1)), font_size!(16.0), z_index, color),
            children: Vec::new(),
            background_object: None,
            z_index,
            cursor_blink_delta: Instant::now(),
            cursor_rectangle: Rectangle::new(0, 0, 1, 16, (255, 255, 255, 255), (z_index + 0.01).min(1.0)),
            uid: "".to_string()
        }
    }
    
    pub fn set_bg_color(&mut self, color: (u8, u8, u8, u8)) {
        
        self.background_object = Some(
            Box::new(
                Rectangle::new(self.position.0, self.position.1, self.size.0, self.size.1, color, self.z_index - 0.0001)
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
            self.cursor_blink_delta = Instant::now();
        }

        if self.selected {
            if self.handler.process(app) {
                self.cursor_blink_delta = Instant::now();
            }
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


        self.text.position = (self.position.0 + 5, self.position.1);

        // app.camera.push();
        app.camera.viewport = (self.position.0, self.position.1, (self.position.0 + self.size.0 as i32 + 25) as u32, (self.position.1 + self.size.1 as i32) as u32);
        self.text.update(app);
        // app.camera.pop();
        
        
        if self.selected {
            let d = self.cursor_blink_delta.elapsed().as_secs_f64();
            if d % 1.0 <= 0.5 {
                let p = self.text.get_draw_offset(app, self.handler.cursor.idx).unwrap();
                self.cursor_rectangle.position = (p.0 as i32 + 3 + self.position.0, p.1 as i32 + 2 + self.position.1);
                self.cursor_rectangle.update(app);
                
                for cursor in &self.handler.cursors {
                    let p = self.text.get_draw_offset(app, cursor.idx).unwrap();
                    self.cursor_rectangle.position = (p.0 as i32 + 3 + self.position.0, p.1 as i32 + 2 + self.position.1);
                    self.cursor_rectangle.update(app);
                }
                        
            }
        }

        app.camera.pop();

    }

    fn get_named_child(&self, path: VecDeque<&str>) -> Option<&mut dyn Component> {
        None
    }

    fn get_element_name(&self) -> &str {
        &self.uid
    }


    fn destroy(self) {
    }
}


