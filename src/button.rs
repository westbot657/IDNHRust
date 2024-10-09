use crate::component::Component;



pub struct Button {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub children: Vec<Box<dyn Component>>,
    pub pressed: bool,
    pub hovered: bool
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, children: Vec<Box<dyn Component>>) -> Self {
        Self {
            position: (x, y),
            size: (width, height),
            children,
            pressed: false,
            hovered: false
        }
    }
}


impl Component for Button {
    fn update(&mut self, app: &mut crate::app::App) {
        
        if app.collides((self.position.0, self.position.1, self.size.0, self.size.1), app.mouse.position) {
            if app.mouse.left_down && !app.last_mouse.left_down {
                self.pressed = true;
            }
            self.hovered = true;
        } else {
            self.hovered = false;
            self.pressed = false
        }

        app.camera.push();

        app.camera.set_position(self.position.0 as f32 / app.window_size.1 as f32, self.position.1 as f32 / app.window_size.1 as f32);

        for child in &mut self.children {
            child.update(app);
        }

        app.camera.pop();
    }

    fn destroy(self) {
    }
}
