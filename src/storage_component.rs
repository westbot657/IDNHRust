use crate::component::Component;



pub struct StorageComponent {
    pub comps: Vec<Box<dyn Component>>
}

impl StorageComponent {
    pub fn new(comps: Vec<Box<dyn Component>>) -> Self {
        Self {
            comps
        }
    }
}


impl Component for StorageComponent {
    fn update(&mut self, _app: &mut crate::app::App) {
    }

    fn collides(&self, _point: (i32, i32)) -> bool {
        false
    }

    fn destroy(self) {
    }
}
