use crate::app::App;


pub trait Component {
    fn update(&self, app: &mut App);

    fn destroy(self);
}

