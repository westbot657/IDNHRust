use std::collections::VecDeque;
use crate::component::Component;



pub struct Collider {
    pub position: (i32, i32),
    pub size: (u32, u32)
}

impl Collider {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            position: (x, y),
            size: (w, h)
        }
    }
}

impl Component for Collider {
    fn update(&mut self, _app: &mut crate::app::App) {}

    fn get_named_child(&self, path: VecDeque<&str>) -> Option<&mut dyn Component> {
        None
    }

    fn get_element_name(&self) -> &str {
        "collider"
    }


    fn destroy(self) {}
}


