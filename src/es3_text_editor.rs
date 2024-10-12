use std::collections::VecDeque;
use crate::app::App;
use crate::component::Component;
use crate::es3::ES3Compiler;
use crate::text_input_handler::TextInputHandler;

struct ES3TextEditor {
    compiler: ES3Compiler,
    text_input_handler: TextInputHandler,
    pub position: (i32, i32),
    pub size: (u32, u32),
    uid: String
}

impl Component for ES3TextEditor {
    fn update(&mut self, app: &mut App) {
        todo!()
    }

    fn get_named_child(&self, path: VecDeque<&str>) -> Option<&mut dyn Component> {
        None
    }

    fn get_element_name(&self) -> &str {
        todo!()
    }


    fn destroy(self) {
    }
}
