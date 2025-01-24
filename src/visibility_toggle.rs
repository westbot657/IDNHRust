use crate::app::App;
use crate::component::Component;
use crate::image::Image;

pub struct VisibilityToggle {
    pub position: (i32, i32),
    pub visible: bool,
    pub group: String,
    pub width: u32,
    visible_texture: Image,
    hidden_texture: Image,
}

impl VisibilityToggle {
    pub fn new(app: &App, group: &str) -> Self {
        
        Self {
            position: (0, 0),
            visible: true,
            group: group.to_string(),
            width: 50,
            visible_texture: Image::new(0, 0, 50, 50, "assets/textures/button/visibility_selector/".to_string() + group + "_shown.png", (0, 0, 50, 50), -0.99).with_shader(app.shaders.prox_fade_texture_white),
            hidden_texture: Image::new(0, 0, 50, 50, "assets/textures/button/visibility_selector/".to_string() + group + "_hidden.png", (0, 0, 50, 50), -0.99).with_shader(app.shaders.prox_fade_texture_white),
        }
    }
    pub fn spacer(app: &App) -> Self {
        Self {
            position: (0, 0),
            visible: false,
            group: "".to_string(),
            width: 10,
            visible_texture: Image::new(0, 0, 10, 50, "assets/textures/visibility_selector_spacer.png", (0, 0, 10, 50), -0.99).with_shader(app.shaders.prox_fade_texture_white),
            hidden_texture: Image::new(0, 0, 10, 50, "assets/textures/visibility_selector_spacer.png", (0, 0, 10, 50), -0.99).with_shader(app.shaders.prox_fade_texture_white),
        }
    }
    
    pub fn is_spacer(&self) -> bool {
        self.group.is_empty()
    }
}



impl Component for VisibilityToggle {
    fn update(&mut self, app: &mut App) {
        self.visible_texture.position = self.position;
        self.hidden_texture.position = self.position;
        if self.visible {
            self.visible_texture.update(app);
        } else {
            self.hidden_texture.update(app);
        }
    }

}