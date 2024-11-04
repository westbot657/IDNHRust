use crate::{app::App, component::Component, editor_app::EditorApp, game_app::GameApp, image::Image};
use crate::component_system::{CompRef, SystematicComponent};

pub struct AppSelector {
    pub game_app: CompRef<GameApp>,
    pub editor_app: CompRef<EditorApp>,

    pub selected_app: u8,

    game_app_unselected: Image,
    game_app_selected: Image,
    
    editor_app_unselected: Image,
    editor_app_selected: Image,

    settings_app_unselected: Image,
    settings_app_selected: Image,
    
    uid: String,
}

impl AppSelector {
    pub fn new(app: &mut App) -> Self {


        Self {
            game_app: GameApp::new().systemize(&mut app.component_system),
            editor_app: EditorApp::new(app).systemize(&mut app.component_system),

            selected_app: 0,

            game_app_unselected: Image::new(
                0, 0, 50, 50, "assets/textures/button/game_app_unselected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ).with_shader(app.shaders.prox_fade_texture_white),
            game_app_selected: Image::new(
                0, 0, 50, 50, "assets/textures/button/game_app_selected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ),

            editor_app_unselected: Image::new(
                0, 50, 50, 50, "assets/textures/button/editor_app_unselected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ).with_shader(app.shaders.prox_fade_texture_white),
            editor_app_selected: Image::new(
                0, 50, 50, 50, "assets/textures/button/editor_app_selected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ),

            settings_app_unselected: Image::new(
                0, app.window_size.1 as i32 - 90, 50, 50, "assets/textures/button/settings_app_icon_unselected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ).with_shader(app.shaders.prox_fade_texture_white),
            settings_app_selected: Image::new(
                0, app.window_size.1 as i32 - 90, 50, 50, "assets/textures/button/settings_app_icon_selected.png".to_string(),
                (0, 0, 50, 50), 0.91
            ),

            uid: "app_selector".to_string(),
        }
    }
}


impl Component for AppSelector {
    fn update(&mut self, app: &mut App) {


        let cam_pos = (50, 0);
        


        if self.selected_app == 0 {
            self.game_app_selected.update(app);

            
            app.camera.push();

            app.camera.set_ipos(50, 0);
            app.camera.translate(cam_pos.0 as f32, cam_pos.1 as f32, app.window_size);
            app.camera.viewport = (45, 0, app.window_size.0 - 45, app.window_size.1);

            self.game_app.update(app);
            app.camera.pop();


        } else {
            self.game_app_unselected.update(app);
        }

        if self.selected_app == 1 {
            self.editor_app_selected.update(app);

            
            app.camera.push();

            app.camera.set_ipos(50, 0);

            app.camera.translate(cam_pos.0 as f32, cam_pos.1 as f32, app.window_size);

            app.camera.viewport = (50, 0, app.window_size.0 - 50, app.window_size.1);
            
            self.editor_app.update(app);
            app.camera.pop();


        } else {
            self.editor_app_unselected.update(app);
        }

        self.settings_app_selected.set_position(0, app.window_size.1 as i32 - 91);
        self.settings_app_unselected.set_position(0, app.window_size.1 as i32 - 91);

        if self.selected_app == 2 {
            self.settings_app_selected.update(app);

            
            app.camera.push();

            app.camera.set_ipos(50, 0);

            app.camera.translate(cam_pos.0 as f32, cam_pos.1 as f32, app.window_size);

            app.camera.viewport = (45, 0, app.window_size.0 - 45, app.window_size.1);
            
            // self.editor_app.update(app);
            app.camera.pop();


        } else {
            self.settings_app_unselected.update(app);
        }


        if app.mouse.left_down {
            if app.collides((0, 0, 50, 50), app.mouse.position) {
                self.selected_app = 0;
            }
            else if app.collides((0, 50, 50, 50), app.mouse.position) {
                self.selected_app = 1;
            }
            else if app.collides((self.settings_app_selected.position.0, self.settings_app_selected.position.1, 50, 50), app.mouse.position) {
                self.selected_app = 2;
            }
        }

    }

}