use crate::app::App;
use crate::component::Component;
use crate::macros::collides;
use crate::text::Text;
use crate::text_input_handler::{IdxSize, TextInputHandler};

pub struct Textbox {
    handler: TextInputHandler,
    position: (i32, i32),
    size: (u32, u32),
    selected: bool,
    hovered: bool,
    text: Text,
}


impl Textbox {
    pub fn new(position: (i32, i32), size: (u32, u32), content: &str, allow_newlines: bool, max_length: Option<IdxSize>, allow_editing: bool, z_index: f32, color: (u8, u8, u8, u8)) -> Self {

        Self {
            handler: TextInputHandler::new(content.to_string(), allow_newlines, max_length, allow_editing),
            position,
            size,
            selected: false,
            hovered: false,
            text: Text::new(0, 0, "", Some(size), 16.0/50.0, z_index, color)
        }
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

        self.text.content = &self.handler.content;

        app.camera.push();

        app.camera.set_ipos(self.position.0, self.position.1);

        self.text.update(app);

        app.camera.pop();

    }

    fn destroy(self) {
    }
}


