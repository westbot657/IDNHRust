use std::process::{Child, Command};

use enigo::Mouse;

use crate::{
    app::App,
    button::Button,
    collider::Collider,
    component::Component,
    image::Image,
    macros::{cast_component, collides, SETTINGS},
    rectangle::Rectangle,
    storage_component::StorageComponent,
    text::Text
};
use crate::macros::font_size;

pub struct WindowFrame {
    children: Vec<Box<dyn Component>>,
    grab_delta: (i32, i32),
    grabbed: bool,
    left_drag: Collider,
    left_corner_drag: Collider,
    bottom_drag: Collider,
    right_corner_drag: Collider,
    right_drag: Collider,
    selected_drag: u32,
    ghost_window: Option<Child>,
    disable_ghost: bool,
    snapped: bool
}


impl WindowFrame {
    pub fn new(title: &str, icon: Image, window_size: (u32, u32), app: &App) -> Self {
        const FRAME_COLOR: (u8, u8, u8, u8) = SETTINGS!(bg medium 4 u8);


        Self {
            children: vec![
                Box::new(Rectangle::new(0, 0, window_size.0, 25, FRAME_COLOR, 0.9).with_shader(app.shaders.prox_fade)),
                Box::new(Rectangle::new(0, 25, 5, window_size.1-45, FRAME_COLOR, 0.9).with_shader(app.shaders.prox_fade)),
                Box::new(Rectangle::new(window_size.0 as i32 - 5, 25, 5, window_size.1-45, FRAME_COLOR, 0.9).with_shader(app.shaders.prox_fade)),
                Box::new(Rectangle::new(0, window_size.1 as i32 - 20, window_size.0, 20, FRAME_COLOR, 0.9).with_shader(app.shaders.prox_fade)),

                Box::new(Button::new(
                    window_size.0 as i32-40, 0, 40, 25,
                    vec![
                        Box::new(Image::new(
                            0, 0, 40, 25, "assets/textures/button/close_app.png".to_string(),
                            (0, 0, 40, 25), 0.91
                        )
                        .with_shader(app.shaders.prox_fade_red)
                        ),
                    ],
                    "close_window"
                )),

                Box::new(Button::new(
                    window_size.0 as i32 - 80, 0, 40, 25,
                    vec![
                        Box::new(Image::new(
                            0, 0, 40, 25, "assets/textures/button/fullscreen.png".to_string(),
                            (0, 0, 40, 25), 0.91
                        )
                        .with_shader(app.shaders.prox_fade_texture_white)
                        )
                    ],
                    "fullscreen"
                )),

                Box::new(Button::new(
                    window_size.0 as i32 - 120, 0, 40, 25,
                    vec![
                        Box::new(Image::new(
                            0, 0, 40, 25, "assets/textures/button/minimize_app.png".to_string(),
                            (0, 0, 40, 25), 0.91
                        )
                        .with_shader(app.shaders.prox_fade_texture_white)
                        )
                    ],
                    "minimize"
                )),

                Box::new(StorageComponent::new(vec![
                    Box::new(Image::new(
                        0, 0, 40, 25, "assets/textures/button/windowed.png".to_string(),
                        (0, 0, 40, 25), 0.91
                    )
                    .with_shader(app.shaders.prox_fade_texture_white)
                    )
                ], "windowing_storage"
                )),

                Box::new(icon),
                Box::new(
                    Text::new(26, 2, title, (None, None, None, None), font_size!(16.0), 0.91, (255, 255, 255, 255))
                )
            ],
            grab_delta: (0, 0),
            grabbed: false,
            left_drag: Collider::new(0, 0, 5, 5),
            left_corner_drag: Collider::new(0, 5, 5, 5),
            bottom_drag: Collider::new(5, 5, 10, 5),
            right_corner_drag: Collider::new(5, 5, 5, 5),
            right_drag: Collider::new(5, 25, 5, 30),
            selected_drag: 0,
            ghost_window: None,
            disable_ghost: false,
            snapped: false
        }
    }

    fn detect_edge_collision(&self, app: &App) -> Option<(i32, i32, u32, u32, u8)> {

        let (mx, my) = app.enigo.location().unwrap();

        for monitor in &app.monitors {
            if monitor.0 <= mx && mx <= monitor.0 + monitor.2 as i32 &&
                monitor.1 <= my && my <= monitor.1 + monitor.3 as i32 {
                if monitor.0 <= mx && mx <= monitor.0 + 5 {
                    if monitor.1 <= my && my <= monitor.1 + 40 {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 7))

                    }
                    else if monitor.1 + monitor.3 as i32 - 40 <= my && my <= monitor.1 + monitor.3 as i32 {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 5))

                    }
                    else {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 6))

                    }
                }
                else if monitor.1 <= my && my <= monitor.1 + 5 {
                    if monitor.0 + monitor.2 as i32 - 40 <= mx && mx <= monitor.0 + monitor.2 as i32 {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 1))

                    }
                    else {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 0))
                    }
                }
                else if monitor.0 + monitor.2 as i32 - 5 <= mx && mx <= monitor.0 + monitor.2 as i32 {
                    if monitor.1 + monitor.3 as i32 - 40 <= my && my <= monitor.1 + monitor.3 as i32 {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 3))

                    }
                    else {
                        return Some((monitor.0, monitor.1, monitor.2, monitor.3, 2))
                        
                    }

                }
                else if monitor.1 + monitor.3 as i32 - 5 <= my && my <= monitor.1 + monitor.3 as i32 {
                    return Some((monitor.0, monitor.1, monitor.2, monitor.3, 4))

                }
            }
        }

        None
    }

}

impl Component for WindowFrame {
    fn update(&mut self, app: &mut App) {


        let top_bar = cast_component!(self.children.get_mut(0).unwrap() => mut Rectangle);
        top_bar.set_size(app.window_size.0, 25);

        let top_bar = cast_component!(self.children.first().unwrap() => Rectangle);


        let exit_button = cast_component!(self.children.get(4).unwrap() => Button);
        let fullscreen_button = cast_component!(self.children.get(5).unwrap() => Button);
        let minimize_button = cast_component!(self.children.get(6).unwrap() => Button);
        

        if app.mouse.left_down && collides!(app, top_bar, app.mouse.position) &&
            !(
                collides!(app, exit_button, app.mouse.position) ||
                collides!(app, fullscreen_button, app.mouse.position) ||
                collides!(app, minimize_button, app.mouse.position) ||
                collides!(app, self.left_drag, app.mouse.position)
            ) && !app.fullscreen {
            let (ax, ay) = app.enigo.location().unwrap();

            self.grab_delta = (ax-app.window_pos.0, ay-app.window_pos.1);
            self.grabbed = true;
            app.mouse.left_down = false; // Set to false to block input to anything behind

            if self.snapped {
                self.snapped = false;
                let dw = app.window.minimum_size().0 as f32 / app.window_size.0 as f32;
                self.grab_delta.0 = (self.grab_delta.0 as f32 * dw) as i32;
                app.set_size(app.window.minimum_size());
            }

        }
        if app.mouse.left_up {
            self.grabbed = false;
            self.selected_drag = 0;
            app.window.set_always_on_top(false);

            if self.ghost_window.is_some() {
                let mut process = self.ghost_window.take().unwrap();
                if let Err(e) = process.kill() {
                    eprintln!("FAILED TO KILL PROCESS {}", e);
                }

                if let Some((pos_x, pos_y, width, height, side)) = self.detect_edge_collision(app) {
                    let target;
                    let width = width as i32;
                    let height = height as i32;

                    if side == 0 {
                        target = (pos_x, pos_y, width, height);
                        app.fullscreen = true;
                        app.pre_fullscreen_pos = (app.window_pos.0, app.window_pos.1 + 20);
                        app.pre_fullscreen_size = app.window_size;
                    }
                    else if side == 1 {
                        target = (pos_x+width/2, pos_y, width/2, height/2);
                    }
                    else if side == 2 {
                        target = (pos_x+width/2, pos_y, width/2, height);
                    }
                    else if side == 3 {
                        target = (pos_x+width/2, pos_y+height/2, width/2, height/2);
                    }
                    else if side == 4 {
                        target = (pos_x, pos_y+height/2, width, height/2);
                    }
                    else if side == 5 {
                        target = (pos_x, pos_y+height/2, width/2, height/2);
                    }
                    else if side == 6 {
                        target = (pos_x, pos_y, width/2, height);
                    }
                    else {
                        target = (pos_x, pos_y, width/2, height/2);
                    }

                    app.set_pos(target.0, target.1);

                    app.set_size((target.2 as u32, target.3 as u32));

                    if side == 0 {
                        let fullscreen_button = cast_component!(self.children.get_mut(5).unwrap() => mut Button);

                        let img: Box<dyn Component> = fullscreen_button.children.pop().unwrap();

                        let storage = cast_component!(self.children.get_mut(7).unwrap() => mut StorageComponent);
                        let img2 = storage.comps.pop().unwrap();
                        storage.comps.push(img);

                        let fullscreen_button = cast_component!(self.children.get_mut(5).unwrap() => mut Button);
                        fullscreen_button.children.push(img2);
                    } else {
                        self.snapped = true;
                    }
                }

            }
        }

        if self.grabbed {
            app.window.raise();
            let loc = app.enigo.location().unwrap();
            app.set_pos(
                loc.0 - self.grab_delta.0,
                loc.1 - self.grab_delta.1
            );

            let edge = self.detect_edge_collision(app);
            if edge.is_some() {
                if self.ghost_window.is_none() && !self.disable_ghost {

                    let this_exe = std::env::current_exe();
                    if this_exe.is_ok() {
                        let rect = edge.unwrap();
                        match Command::new(this_exe.unwrap())
                        .args(["--win-ghost", &format!("{}", rect.0), &format!("{}", rect.1), &format!("{}", rect.2), &format!("{}", rect.3), &format!("{}", rect.4)])
                        .spawn() {
                            Ok(child) => {
                                self.ghost_window = Some(child);
                            }
                            Err(_e) => {
                                self.disable_ghost = true; // if something goes wrong, then prevent retrying until later
                            }
                        }
                    }

                    app.window.raise();
                    app.window.set_always_on_top(true);
                }
            } else if self.ghost_window.is_some() {
                let mut process = self.ghost_window.take().unwrap();
                if let Err(e) = process.kill() {
                    eprintln!("FAILED TO KILL PROCESS {}", e);
                }

            } else {
                self.disable_ghost = false;
            }

        }

        if self.selected_drag != 0 {
            let (mx, my) = app.enigo.location().unwrap();
            if self.selected_drag == 1 || self.selected_drag == 2 {
                app.set_pos(mx.min(self.grab_delta.1 - app.window.minimum_size().0 as i32), app.window_pos.1);
                app.set_size(((app.window.minimum_size().0).max((self.grab_delta.1 - mx).max(0) as u32), app.window_size.1));
            }

            if self.selected_drag == 2 || self.selected_drag == 3 || self.selected_drag == 4 {
                app.set_size((app.window_size.0, (app.window.minimum_size().1).max((my - app.window_pos.1).max(0) as u32)));
            }

            if self.selected_drag == 4 || self.selected_drag == 5 {
                app.set_size(((app.window.minimum_size().0).max((mx - app.window_pos.0).max(0) as u32), app.window_size.1));
            }

        }

        let left_bar = cast_component!(self.children.get_mut(1).unwrap() => mut Rectangle);
        left_bar.set_size(5, app.window_size.1-45);

        let right_bar = cast_component!(self.children.get_mut(2).unwrap() => mut Rectangle);
        right_bar.set_position(app.window_size.0 as i32 - 5, 25);
        right_bar.set_size(5, app.window_size.1-45);

        let bottom_bar = cast_component!(self.children.get_mut(3).unwrap() => mut Rectangle);
        bottom_bar.set_position(0, app.window_size.1 as i32 - 20);
        bottom_bar.set_size(app.window_size.0, 20);

        let exit_button = cast_component!(self.children.get_mut(4).unwrap() => mut Button);
        exit_button.position = (app.window_size.0 as i32 - 40, 0);

        let fullscreen_button = cast_component!(self.children.get_mut(5).unwrap() => mut Button);
        fullscreen_button.position = (app.window_size.0 as i32 - 80, 0);
        
        let minimize_button = cast_component!(self.children.get_mut(6).unwrap() => mut Button);
        minimize_button.position = (app.window_size.0 as i32 - 120, 0);

        self.left_drag.size.1 = app.window_size.1-5;
        self.left_corner_drag.position.1 = app.window_size.1 as i32 - 5;
        self.bottom_drag.position.1 = app.window_size.1 as i32 - 5;
        self.bottom_drag.size.0 = app.window_size.0 - 10;
        self.right_corner_drag.position = (app.window_size.0 as i32 - 5, app.window_size.1 as i32 - 5);
        self.right_drag.position.0 = app.window_size.0 as i32 - 5;
        self.right_drag.size.1 = app.window_size.1 - 30;

        for child in &mut self.children {
            child.update(app);
        }

        if !app.fullscreen {
            if collides!(app, self.left_drag, app.mouse.position) {
                app.set_cursor("SizeWE".to_string());
                if app.mouse.left_down {
                    self.selected_drag = 1;
                    self.grab_delta.1 = app.window_pos.0 + app.window_size.0 as i32;
                }
            }

            else if collides!(app, self.left_corner_drag, app.mouse.position) {
                app.set_cursor("SizeNESW".to_string());
                if app.mouse.left_down {
                    self.selected_drag = 2;
                    self.grab_delta.1 = app.window_pos.0 + app.window_size.0 as i32;
                }
            }

            else if collides!(app, self.bottom_drag, app.mouse.position) {
                app.set_cursor("SizeNS".to_string());
                if app.mouse.left_down {
                    self.selected_drag = 3;
                }
            }

            else if collides!(app, self.right_corner_drag, app.mouse.position) {
                app.set_cursor("SizeNWSE".to_string());
                if app.mouse.left_down {
                    self.selected_drag = 4;
                }
            }

            else if collides!(app, self.right_drag, app.mouse.position) {
                app.set_cursor("SizeWE".to_string());
                if app.mouse.left_down {
                    self.selected_drag = 5;
                }
            }

        }

        let exit_button = cast_component!(self.children.get(4).unwrap() => Button);

        if exit_button.pressed {
            app.should_quit = true;
        }

        let fullscreen_button = cast_component!(self.children.get_mut(5).unwrap() => mut Button);

        if fullscreen_button.pressed {
            if app.fullscreen {
                app.set_size(app.pre_fullscreen_size);
                app.set_pos(app.pre_fullscreen_pos.0, app.pre_fullscreen_pos.1);
                // app.window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                app.fullscreen = false;
            } else {
                // app.window.set_fullscreen(sdl2::video::FullscreenType::True).unwrap();
                app.pre_fullscreen_pos = app.window_pos;
                app.pre_fullscreen_size = app.window_size;

                let space = app.get_monitor_with_cursor().unwrap();

                app.set_pos(space.0, space.1);
                app.set_size((space.2, space.3));
                app.fullscreen = true;
            }

            let img: Box<dyn Component> = fullscreen_button.children.pop().unwrap();

            let storage = cast_component!(self.children.get_mut(7).unwrap() => mut StorageComponent);
            let img2 = storage.comps.pop().unwrap();
            storage.comps.push(img);

            let fullscreen_button = cast_component!(self.children.get_mut(5).unwrap() => mut Button);
            fullscreen_button.children.push(img2);

        }

        let minimize_button = cast_component!(self.children.get(6).unwrap() => Button);

        if minimize_button.pressed && !app.window.is_minimized() {
            app.window.minimize();
        }



    }

    fn destroy(self) {
    }
}


