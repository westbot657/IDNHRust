use std::collections::HashMap;
use crate::{canvas::Canvas, component::Component, rectangle::Rectangle};
use crate::es3::style_flags;
use crate::text::Text;

pub struct EditorApp {
    canvas: Canvas,
    visibility_toggles: HashMap<String, bool>,
    children: Vec<Box<dyn Component>>,
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

        let mut children: Vec<Box<dyn Component>> = Vec::new();

        children.push(Box::new(Text::new(1, 2, "Not Italic".to_string(), None, 25.0/50.0, 1.0, (255, 255, 255, 0))));
        let mut txt = Text::new(1, 8, "Italic".to_string(), None, 25.0/50.0, 1.0, (255, 255, 255, 0));
        txt.set_styles(style_flags::ITALIC);
        children.push(Box::new(txt));

        Self {
            canvas,
            visibility_toggles,
            children,
        }
    }
}


impl Component for EditorApp {
    fn update(&mut self, app: &mut crate::app::App) {

        self.canvas.size = (app.window_size.0 - 360, app.window_size.1 - 100);

        // self.canvas.update(app);

        for child in &mut self.children {
            child.update(app);
        }

    }

    fn destroy(self) {
    }
}
