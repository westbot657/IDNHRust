use crate::{canvas::Canvas, component::Component, rectangle::Rectangle};



pub struct EditorApp {
    canvas: Canvas
}


impl EditorApp {
    pub fn new() -> Self {

        let mut canvas = Canvas::new(0, 0, 500, 500, 50, 0.5, (255, 255, 255, 255));

        canvas.children.push(
            Box::new(
                Rectangle::new(0, 0, 25, 50, (255, 0, 0, 127), 1.0)
            )
        );

        Self {
            canvas
        }
    }
}


impl Component for EditorApp {
    fn update(&mut self, app: &mut crate::app::App) {

        self.canvas.update(app);

    }

    fn destroy(self) {
    }
}
