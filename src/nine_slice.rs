use crate::app::App;
use crate::component::Component;
use crate::image::Image;

pub struct NineSliceBase {
    src: String,
    src_size: (u32, u32),
    slices_x: (u32, u32),
    slices_y: (u32, u32),
    textures: ((Image, Image, Image), (Image, Image, Image), (Image, Image, Image)),
    z_index: f32
}

impl NineSliceBase {
    pub fn new(app: &mut App, src: impl ToString, src_size: (u32, u32), slices_x: (u32, u32), slices_y: (u32, u32), z_index: f32) {
        let base = Self {
            src: src.to_string(),
            src_size,
            slices_x,
            slices_y,
            textures: (
                (
                    Image::new(0, 0, slices_x.0, slices_y.0, src.to_string(), (0, 0, slices_x.0, slices_y.0), z_index),
                    Image::new(0, 0, slices_x.0, slices_y.1 - slices_y.0, src.to_string(), (0, slices_y.0, slices_x.0, slices_y.1 - slices_y.0), z_index),
                    Image::new(0, 0, slices_x.0, src_size.1 - slices_y.1, src.to_string(), (0, slices_y.1, slices_x.0, src_size.1 - slices_y.1), z_index),
                ),
                (
                    Image::new(0, 0, slices_x.1 - slices_x.0, slices_y.0, src.to_string(), (slices_x.0, 0, slices_x.1 - slices_x.0, slices_y.0), z_index),
                    Image::new(0, 0, slices_x.1 - slices_x.0, slices_y.1 - slices_y.0, src.to_string(), (slices_x.0, slices_y.0, slices_x.1 - slices_x.0, slices_y.1 - slices_y.0), z_index),
                    Image::new(0, 0, slices_x.1 - slices_x.0, src_size.1 - slices_y.1, src.to_string(), (slices_x.0, slices_y.1, slices_x.1 - slices_x.0, src_size.1 - slices_y.1), z_index),
                ),
                (
                    Image::new(0, 0, src_size.0 - slices_x.1, slices_y.0, src.to_string(), (slices_x.1, 0, src_size.0 - slices_x.1, slices_y.0), z_index),
                    Image::new(0, 0, src_size.0 - slices_x.1, slices_y.1 - slices_y.0, src.to_string(), (slices_x.1, slices_y.0, src_size.0 - slices_x.1, slices_y.1 - slices_y.0), z_index),
                    Image::new(0, 0, src_size.0 - slices_x.1, src_size.1 - slices_y.1, src.to_string(), (slices_x.1, slices_y.1, src_size.0 - slices_x.1, src_size.1 - slices_y.1), z_index),
                )
            ),
            z_index
        };
        
        app.nine_slices.insert(src.to_string(), base);
        
    }
    
    pub fn render(&self, slice: &NineSlice) {
        
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
        let slice_base = app.nine_slices.get(&self.src).unwrap();
        slice_base.render(self);
    }
}
