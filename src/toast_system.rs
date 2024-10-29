use std::collections::VecDeque;
use std::time::Instant;
use crate::app::App;
use crate::component::Component;
use crate::easing::ease_in_out_sine;
use crate::image::Image;
use crate::macros::font_size;
use crate::text::Text;

pub struct Toast {
    pub title: String,
    pub components: Vec<Box<dyn Component>>,
    pub creation_time: Instant,
    pub duration: f32,
    pub fade_time: f32
}

pub struct ToastSystem {
    toasts: Vec<Toast>,
    width: u32,
    toast_height: u32,
    toast_background: Image,
    toast_text: Text,
}

impl ToastSystem {
    pub fn new(app: &App, width: u32, toast_height: u32) -> Self {
        
        Self {
            toasts: Vec::new(),
            width,
            toast_height,
            toast_background: Image::new(0, 0, 300, 80, "assets/textures/toast_background.png", (0, 0, 300, 80), 0.99).with_shader(app.shaders.prox_fade_texture_white),
            toast_text: Text::new(5, 0, "", (None, None, Some(290), None), font_size!(16.0), 0.999, (255, 255, 255, 255))
        }
        
    }
    
    pub fn blank() -> Self {
        Self {
            toasts: Vec::new(),
            width: 1,
            toast_height: 1,
            toast_background: Image::new(0, 0, 300, 80, "assets/textures/toast_background.png", (0, 0, 300, 80), 0.99),
            toast_text: Text::new(5, 0, "", (None, None, Some(290), None), font_size!(16.0), 0.999, (255, 255, 255, 255))
        }
    }
    
    /// components is a vec of any components to attach to the displayed toast.
    /// duration defaults to 8.0 secs if None is passed
    /// fade_time will default to 1.0 seconds
    pub fn push(&mut self, text: impl ToString, components: Vec<Box<dyn Component>>, duration: Option<f32>, fade_time: Option<f32>) {
        let toast = Toast {
            title: text.to_string(),
            components,
            creation_time: Instant::now(),
            duration: duration.unwrap_or(8.0) + (fade_time.unwrap_or(1.0) * 2.0),
            fade_time: fade_time.unwrap_or(1.0)
        };
        self.toasts.push(toast);
    }
}

impl Component for ToastSystem {
    fn update(&mut self, app: &mut App) {
        let mut to_remove = Vec::new();
        let mut i = 0;
        let l = self.toasts.len();
        for toast in &mut self.toasts {
            let t = toast.creation_time.elapsed().as_secs_f32();
            if t > toast.duration {
                to_remove.push(i);
            } else {
                let dx: i32;
                if t > toast.duration - toast.fade_time {
                    dx = ((self.width + 10) as f32 - (ease_in_out_sine(t - (toast.duration - toast.fade_time)) * (self.width + 10) as f32)) as i32;
                } else if t > toast.fade_time {
                    dx = self.width as i32 + 10;
                } else {
                    dx = (ease_in_out_sine(t) * (self.width + 10) as f32) as i32;
                }
                
                // self.toast_background.position = (app.window_size.0 as i32 - dx, app.window_size.1 as i32 - (self.toast_height as i32 + 10) * (i as i32));
                app.camera.push();
                app.camera.set_ipos(app.window_size.0 as i32 - dx, app.window_size.1 as i32 - 110 - (self.toast_height as i32 + 10) * (i as i32));
                app.camera.translate((app.window_size.0 as i32 - dx) as f32, (app.window_size.1 as i32 - 110 - (self.toast_height as i32 + 10) * (i as i32)) as f32, app.window_size);
                self.toast_background.update(app);
                self.toast_text.content = toast.title.clone();
                self.toast_text.update(app);
                
                app.camera.pop();
                
            }
            
            i += 1;
        }
        
        let mut off = 0;
        for i in to_remove {
            self.toasts.remove(i-off);
            off += 1;
        }
        
    }

    fn destroy(self) {
        
    }
}

