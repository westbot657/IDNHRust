use std::collections::VecDeque;
use crate::component::Component;



pub struct GameApp {

}

impl GameApp {
    pub fn new() -> Self {

        Self {

        }
    }
}

impl Component for GameApp {
    fn update(&mut self, app: &mut crate::app::App) {
    }

    fn get_named_child(&self, path: VecDeque<&str>) -> Option<&mut dyn Component> {
        None
    }

    fn get_element_name(&self) -> &str {
        "game"
    }


    fn destroy(self) {
    }
}

