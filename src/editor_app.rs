use std::collections::HashMap;
use crate::{canvas::Canvas, component::Component, rectangle::Rectangle};



pub struct EditorApp {
    canvas: Canvas,
    visibility_toggles: HashMap<String, bool>
}


impl EditorApp {
    pub fn new() -> Self {

        let mut canvas = Canvas::new(0, 0, 500, 500, 50, 0.5, (255, 255, 255, 255));

        canvas.children.push(
            Box::new(
                Rectangle::new(0, 0, 25, 50, (255, 0, 0, 127), 1.0)
            )
        );

        let mut visibility_toggles = HashMap::new();



        Self {
            canvas,
            visibility_toggles
        }
    }
}


impl Component for EditorApp {
    fn update(&mut self, app: &mut crate::app::App) {

        self.canvas.size = (app.window_size.0 - 360, app.window_size.1 - 100);

        self.canvas.update(app);

    }

    fn destroy(self) {
    }
}
