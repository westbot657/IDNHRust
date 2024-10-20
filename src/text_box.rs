use std::collections::VecDeque;
use std::mem;
use std::time::Instant;
use crate::app::App;
use crate::component::Component;
use crate::history_manager::HistoryEvent;
use crate::macros::{cast_component, collides, font_size};
use crate::rectangle::Rectangle;
use crate::text::Text;
use crate::text_input_handler::{IdxSize, TextInputHandler};

pub struct TextTypeHistory {
    uid: String,
    data: String,
}

impl TextTypeHistory {
    pub fn new(uid: impl ToString, data: impl ToString) -> Self {
        Self {
            uid: uid.to_string(),
            data: data.to_string()
        }
    }
}

impl HistoryEvent for TextTypeHistory {
    fn redo(&mut self, app: &mut App) {
        let text_box = cast_component!(app.get_named_child(&self.uid).unwrap() => mut Textbox);
        
        mem::swap(&mut text_box.handler.content, &mut self.data);
    }

    fn undo(&mut self, app: &mut App) {
        self.redo(app)
    }
}

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
    pub uid: String,
    offset: (i32, i32),
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
            uid: "".to_string(),
            offset: (0, 0),
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
    
    pub fn set_offset(&mut self, offset: (i32, i32)) {
        self.offset = offset;
    }
    
    /// Sets the offset to the nearest offset that contains `region` in the text-box's bounding box
    /// if a dimension of region is larger than the bounding box, that axis will be centered
    pub fn encapsulate_region(&mut self, region: (i32, i32, u32, u32), padding: u32) {
        
    }
    
}


impl Component for Textbox {
    fn update(&mut self, app: &mut App) {
        self.hovered = collides!(app, self, app.mouse.position);

        self.set_offset(app.mouse.position);
        
        self.text.bounds = (Some(-self.offset.0), Some(-self.offset.1), Some((self.size.0 as i32 - self.offset.0).max(0) as u32), Some((self.size.1 as i32 - self.offset.1).max(0) as u32));
        
        if self.hovered {
            app.set_cursor("IBeam".to_string());
        }
        
        if app.mouse.left_down {
            self.selected = self.hovered;
            if self.hovered {
                self.cursor_blink_delta = Instant::now();
                
                let dx = self.position.0;
                
            }
        }

        if self.selected {
            if self.handler.process(app) {
                self.cursor_blink_delta = Instant::now();
            }
            
            if self.handler.should_focus_cursor() {
                let (mut dx, mut dy) = self.handler.get_text_pos(self.handler.cursor.idx).unwrap();
                
                let mut w = 0;
                let mut h = 0;
                
                app.font_handler.style_flagged(self.text.styles).skip_char(&mut w, &mut 0, " ", self.text.scale);
                app.font_handler.style_flagged(self.text.styles).skip_char(&mut 0, &mut h, "\n", self.text.scale);
                
                dx *= w as IdxSize;
                dy *= h as IdxSize;
            }
            
            if self.handler.should_update_history() {
                let hist = TextTypeHistory::new(app.get_child_path(), &self.handler.content);
                
                app.history.add_history(hist);
            }
            
        }

        self.text.content = self.handler.content.to_string();

        app.camera.push();

        // app.camera.set_ipos(self.position.0, self.position.1);

        if let Some(bg) = &mut self.background_object {
            bg.update(app);
        }
        
        for child in &mut self.children {
            child.update(app);
        }


        self.text.position = (self.position.0 + 5, self.position.1);

        // app.camera.viewport = (self.position.0, self.position.1, (self.position.0 + self.size.0 as i32 + 25) as u32, (self.position.1 + self.size.1 as i32) as u32);
        app.camera.push();
        app.camera.set_ipos(self.offset.0, self.offset.1);
        app.camera.translate(self.offset.0 as f32, self.offset.1 as f32, app.window_size);
        
        // let (_, viewport, _) = app.camera.peek();
        println!("text box rect: {:?}, {:?}", self.position, self.size);
        let vp = app.camera.map_rect((self.position.0, self.position.1, self.size.0, self.size.1), app.window_size);
        // app.camera.viewport = (self.position.0 - self.offset.0 + i_pos.0, self.position.1 - self.offset.1 + i_pos.1, self.size.0, self.size.1);
        println!("Calculated viewport: {:?}", vp);
        app.camera.viewport = (vp.0, vp.1, vp.2, vp.3);
        
        self.text.update(app);
        
        
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


