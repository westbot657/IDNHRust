use crate::{app::App, component::Component, rectangle::Rectangle};



pub struct WindowFrame {
    title: String,
    children: Vec<Box<dyn Component>>
}

impl WindowFrame {
    pub fn new(title: String, icon: &str, window_size: (u32, u32)) -> Self {


        Self {
            title,
            children: vec![
                Box::new(Rectangle::new(0, 0, window_size.0, 20, (10, 10, 10, 255), -0.9)),
                Box::new(Rectangle::new(0, 20, 5, window_size.1-40, (10, 10, 10, 255), -0.9)),
                Box::new(Rectangle::new(window_size.0 as i32 - 5, 20, 5, window_size.1-40, (10, 10, 10, 255), -0.9)),
                Box::new(Rectangle::new(0, window_size.1 as i32 - 20, window_size.0, 20, (10, 10, 10, 255), -0.9))
            ]
        }
    }
}

impl Component for WindowFrame {
    fn update(&self, app: &mut App) {
        
        for child in &self.children {
            child.update(app);
        }
    }

    fn destroy(self) {
    }
}


