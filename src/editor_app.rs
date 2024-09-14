use crate::component::Component;



pub struct EditorApp {

}


impl EditorApp {
    pub fn new() -> Self {

        Self {

        }
    }
}


impl Component for EditorApp {
    fn update(&mut self, app: &mut crate::app::App) {
    }

    fn destroy(self) {
    }
}
