use std::collections::VecDeque;
use crate::{canvas::Canvas, component::Component, rectangle::Rectangle};
use crate::macros::font_size;
use crate::text::Text;
use crate::text_box::Textbox;
use crate::visibility_toggle::VisibilityToggle;

pub struct EditorApp {
    canvas: Canvas,
    visibility_toggles: Vec<VisibilityToggle>,
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

        let visibility_toggles = vec![
            VisibilityToggle::new("weapons"),
        ];

        let mut children: Vec<Box<dyn Component>> = Vec::new();

        children.push(Box::new(Text::new(50, 20, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz", (None, None, None, None), font_size!(16.0), 0.99, (255, 255, 255, 255))));
        
        
        let mut text_box = Textbox::new(
            (50, 100), (500, 500),
            "",
            true, None, true,
            1.0, (255, 255, 255, 255)
        );
        
        text_box.set_bg_color((10, 10, 10, 255));

        canvas.children.push(Box::new(text_box));

        // let mut txt = Text::new(50, 60, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz", None, font_size!(16.0), 1.0, (255, 255, 255, 255));
        // txt.set_styles(style_flags::ITALIC);
        // children.push(Box::new(txt));
        //
        // let mut txt = Text::new(50, 100, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz", None, font_size!(16.0), 1.0, (255, 255, 255, 255));
        // txt.set_styles(style_flags::BOLD);
        // children.push(Box::new(txt));
        //
        //
        // let mut txt = Text::new(50, 140, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz", None, font_size!(16.0), 1.0, (255, 255, 255, 255));
        // txt.set_styles(style_flags::BOLD | style_flags::ITALIC);
        // children.push(Box::new(txt));

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

        self.canvas.update(app);
        
        

        for child in &mut self.children {
            child.update(app);
        }

    }



    fn destroy(self) {
    }
}
