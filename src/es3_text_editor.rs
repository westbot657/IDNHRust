use crate::app::App;
use crate::component::Component;
use crate::es3::ES3Compiler;
use crate::text_input_handler::TextInputHandler;

struct ES3TextEditor {
    compiler: ES3Compiler,
    text_input_handler: TextInputHandler,
    pub position: (i32, i32),
    pub size: (u32, u32)
}

impl Component for ES3TextEditor {
    fn update(&mut self, app: &mut App) {
        todo!()
    }

    fn destroy(self) {
    }
}
