use crate::{canvas::Canvas, component::Component};



pub struct EditorApp {
    canvas: Canvas
}


impl EditorApp {
    pub fn new() -> Self {

        Self {
            canvas: Canvas::new(0, 0, 500, 500, 1, 0.5, (255, 255, 255, 255))
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
