use super::widget::Widget;



pub struct Button {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    children: Vec<Box<dyn Widget>>
}

impl Button {
    fn new(x: i32, y: i32, width: u32, height: u32, children:Option<Vec<Box<dyn Widget>>>) -> Button {
        Button {
            x: x,
            y: y,
            width: width,
            height: height,
            children: children.or(Some(vec![])).unwrap()
        }
    }
}

impl Widget for Button {

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn get_position(&self) -> (i32, i32) {
        return (self.x, self.y)
    }

    fn get_size(&self) -> (u32, u32) {
        return (self.width, self.height)
    }

    fn update(&mut self, draw_ctx: &mut crate::draw_context::DrawContext) {
        todo!()
    }

    fn event(&mut self, event_ctx: &mut crate::event_context::EventContext) {
        todo!()
    }
}


