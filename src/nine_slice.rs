use crate::app::App;
use crate::component::Component;

pub struct NineSliceBase {
    src: String,
    slices_x: (u32, u32),
    slices_y: (u32, u32),
}

impl NineSliceBase {
    pub fn new(app: &mut App, src: impl ToString, slices_x: (u32, u32), slices_y: (u32, u32)) {
        let base = Self {
            src: src.to_string(),
            slices_x,
            slices_y,
        };
        
        app.nine_slices.insert(src.to_string(), base);
        
    }
}


pub struct NineSlice {
    pub src: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
}

impl NineSlice {}


impl Component for NineSlice {
    fn update(&mut self, app: &mut App) {
        
    }
}
