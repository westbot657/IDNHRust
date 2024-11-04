use crate::component::Component;



pub struct StorageComponent {
    pub comps: Vec<Box<dyn Component>>,
    pub uid: String
}

impl StorageComponent {
    pub fn new(comps: Vec<Box<dyn Component>>, uid: &str) -> Self {
        Self {
            comps,
            uid: uid.to_string()
        }
    }
}


impl Component for StorageComponent {
    fn update(&mut self, _app: &mut crate::app::App) {
    }

}
