use std::any::Any;

use crate::app::App;

pub trait ComponentToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: ComponentToAny {
    fn update(&mut self, app: &mut App);

    fn destroy(self);
}

