use crate::{app::App, component::Component, editor_app::EditorApp, game_app::GameApp, image::Image};



pub struct AppSelector {
    pub editor_app: EditorApp,
    pub game_app: GameApp,

    pub selected_app: u8,

    editor_app_unselected: Box<Image>,
    editor_app_selected: Box<Image>,
    
    game_app_unselected: Box<Image>,
    game_app_selected: Box<Image>,
}

impl AppSelector {
    pub fn new(app: &App) -> Self {


        Self {
            editor_app: EditorApp::new(),
            game_app: GameApp::new(),

            selected_app: 0,

            editor_app_unselected: Box::new(Image::new(
                0, 50, 50, 50, "assets/textures/button/editor_app_unselected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ).with_shader(app.shaders.prox_fade_texture)),
            editor_app_selected: Box::new(Image::new(
                0, 50, 50, 50, "assets/textures/button/editor_app_selected.png".to_string(),
                (0, 0, 50, 50), 0.91
            )),
            
            game_app_unselected: Box::new(Image::new(
                0, 0, 50, 50, "assets/textures/button/game_app_unselected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ).with_shader(app.shaders.prox_fade_texture)),
            game_app_selected: Box::new(Image::new(
                0, 0, 50, 50, "assets/textures/button/game_app_selected.png".to_string(),
                (0, 0, 50, 50), 0.91
            )),
        }

    }
}


impl Component for AppSelector {
    fn update(&mut self, app: &mut crate::app::App) {
        if self.selected_app == 0 {
            self.game_app_selected.update(app);

            self.game_app.update(app);

        } else {
            self.game_app_unselected.update(app);
        }

        if self.selected_app == 1 {
            self.editor_app_selected.update(app);

            self.editor_app.update(app);

        } else {
            self.editor_app_unselected.update(app);
        }

        if app.mouse.left_down {
            if app.collides((0, 0, 50, 50), app.mouse.position) {
                self.selected_app = 0;
            }
            else if app.collides((0, 50, 50, 50), app.mouse.position) {
                self.selected_app = 1;
            }
        }

    }

    fn destroy(self) {
    }
}