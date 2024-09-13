use crate::{app::App, component::Component, image::Image, macros::cast_component, rectangle::Rectangle, text::Text};


pub struct WindowFrame {
    title: String,
    children: Vec<Box<dyn Component>>
}


impl WindowFrame {
    pub fn new(title: String, icon: Image, window_size: (u32, u32)) -> Self {
        const FRAME_COLOR: (u8, u8, u8, u8) = (30, 30, 30, 255);


        Self {
            title: title.clone(),
            children: vec![
                Box::new(Rectangle::new(0, 0, window_size.0, 25, FRAME_COLOR, 0.9)),
                Box::new(Rectangle::new(0, 25, 5, window_size.1-45, FRAME_COLOR, 0.9)),
                Box::new(Rectangle::new(window_size.0 as i32 - 5, 25, 5, window_size.1-45, FRAME_COLOR, 0.9)),
                Box::new(Rectangle::new(0, window_size.1 as i32 - 20, window_size.0, 20, FRAME_COLOR, 0.9)),

                Box::new(icon),
                Box::new(
                    Text::new(26, 2, title, None, 0.26, 0.91, (255, 255, 255, 255))
                )
            ]
        }
    }
}

impl Component for WindowFrame {
    fn update(&mut self, app: &mut App) {

        

        let top_bar = cast_component!(self.children.get_mut(0).unwrap() => mut Rectangle);
        top_bar.set_size(app.window_size.0, 25);

        let left_bar = cast_component!(self.children.get_mut(1).unwrap() => mut Rectangle);
        left_bar.set_size(5, app.window_size.1-45);

        let right_bar = cast_component!(self.children.get_mut(2).unwrap() => mut Rectangle);
        right_bar.set_position(app.window_size.0 as i32 - 5, 25);
        right_bar.set_size(5, app.window_size.1-45);

        let bottom_bar = cast_component!(self.children.get_mut(3).unwrap() => mut Rectangle);
        bottom_bar.set_position(0, app.window_size.1 as i32 - 20);
        bottom_bar.set_size(app.window_size.0, 20);

        for child in &mut self.children {
            child.update(app);
        }
    }

    fn destroy(self) {
    }
}


