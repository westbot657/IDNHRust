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
    
}


