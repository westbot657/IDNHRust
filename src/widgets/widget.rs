use crate::{draw_context::DrawContext, event_context::EventContext};



pub trait Widget {
    fn set_position(&mut self, x: i32, y: i32);
    fn set_size(&mut self, width: u32, height: u32);
    fn get_position(&self) -> (i32, i32);
    fn get_size(&self) -> (u32, u32);
    fn update(&mut self, draw_ctx: &mut DrawContext);
    fn event(&mut self, event_ctx: &mut EventContext);
    fn add_child(&mut self, child: dyn Widget);
    fn add_children(&mut self, children: Vec<Box<dyn Widget>>);
}

// pub struct Widget {
//     x: i32,
//     y: i32,
//     width: u32,
//     height: u32,
//     children: Box<Vec<Widget>>
// }

