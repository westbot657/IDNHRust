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

    fn collides(&self, _point: (i32, i32)) -> bool {
        false
    }

    fn destroy(self) {
    }
}

