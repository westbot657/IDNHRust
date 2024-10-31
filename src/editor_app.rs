use crate::{canvas::Canvas, component::Component, rectangle::Rectangle};
use crate::app::App;
use crate::image::Image;
use crate::text_box::Textbox;
use crate::visibility_toggle::VisibilityToggle;

pub struct EditorApp {
    canvas: Canvas,
    visibility_toggles: Vec<VisibilityToggle>,
    vis_toggle_bg: Rectangle,
    children: Vec<Box<dyn Component>>,
}


impl EditorApp {
    
    pub fn blank() -> Self {
        Self {
            canvas: Canvas::new(0, 0, 0, 0, 0, 0.0, (0, 0, 0, 0)),
            visibility_toggles: Vec::new(),
            vis_toggle_bg: Rectangle::new(0, 0, 0, 0, (0, 0, 0, 0), 0.0),
            children: Vec::new(),
        }
    }
    
    pub fn new(app: &mut App) -> Self {

        let mut canvas = Canvas::new(0, 0, 500, 500, 50, 0.5, (255, 255, 255, 255));

        canvas.children.push(
            Box::new(
                Rectangle::new(0, 0, 25, 50, (255, 0, 0, 127), 1.0)
            )
        );

        let visibility_toggles = vec![
            VisibilityToggle::new(app, "weapons"),
            VisibilityToggle::new(app, "ammo"),
            VisibilityToggle::new(app, "armor"),
            VisibilityToggle::new(app, "tools"),
            VisibilityToggle::new(app, "items"),
            VisibilityToggle::spacer(app),
            VisibilityToggle::new(app, "rooms"),
            VisibilityToggle::new(app, "roads"),
            VisibilityToggle::spacer(app),
            VisibilityToggle::new(app, "enemies"),
            VisibilityToggle::new(app, "combats"),
            VisibilityToggle::spacer(app),
            VisibilityToggle::new(app, "scripts"),
        ];

        let mut children: Vec<Box<dyn Component>> = Vec::new();

        // children.push(Box::new(Text::new(50, 20, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz", (None, None, None, None), font_size!(16.0), 0.99, (255, 255, 255, 255))));
        
        
        let text_box = Textbox::new(
            app,
            (50, 100), (500, 500),
            "",
            true, None, true,
            0.98, (255, 255, 255, 255)
        );
        
        if let Some(mut textb) = text_box.get(&mut app.component_system) {
            
            textb.set_bg_color((10, 10, 10, 255));
            
            text_box.restore(&mut app.component_system, textb);
        }

        canvas.children.push(Box::new(text_box));

        Self {
            canvas,
            visibility_toggles,
            vis_toggle_bg: Rectangle::new(0, 0, 1, 35, (24, 24, 24, 255), 0.99).with_shader(app.shaders.prox_fade),
            children,
        }
    }
}


impl Component for EditorApp {
    fn update(&mut self, app: &mut App) {

        self.canvas.size = (app.window_size.0 - 360, app.window_size.1 - 100);

        self.canvas.update(app);
        
        

        for child in &mut self.children {
            child.update(app);
        }

        
        let mut dx = 0;
        let dy = app.window_size.1 as i32 - 130;
        for toggle in &mut self.visibility_toggles {
            toggle.position = (dx, dy);
            toggle.update(app);
            dx += toggle.width as i32;
        }
        self.vis_toggle_bg.size.0 = dx as u32;
        self.vis_toggle_bg.position = (0, dy+50);
        self.vis_toggle_bg.update(app);
        
    }



    fn destroy(self) {
    }
}
