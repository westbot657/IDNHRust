extern crate sdl2;
extern crate gl;

mod shaders;
mod texture_atlas;
mod app;
mod component;
mod rectangle;
mod image;
mod text;
mod camera;
mod window_frame;
mod macros;
mod button;
mod storage_component;
mod app_selector;
mod editor_app;
mod game_app;
mod collider;
mod monitor_info;
mod keybinds;
mod settings;
mod easing;
mod canvas;

use std::{collections::VecDeque, env, time::Instant};

use app::App;
use device_query::{DeviceQuery, DeviceState};
use enigo::{Enigo, Mouse, Settings};
use macros::SETTINGS;
use shaders::Shaders;
use sdl2::{event::WindowEvent, image::LoadSurface, mouse::{MouseButton, MouseWheelDirection}, video::GLProfile};
use text::CharAtlas;

fn main() -> Result<(), String> {

    let mut args: VecDeque<String> = env::args().collect();

    while args.len() > 0 {
        let arg = args.pop_front().unwrap();

        if arg == "--win-ghost" {
            const ERR_MSG: &str = "arguments after --win-ghost must be in form 'i32 i32 u32 u32 u8'";
            let pos_x = args.pop_front().expect(ERR_MSG)
                .parse::<i32>().expect(ERR_MSG);
            let pos_y = args.pop_front().expect(ERR_MSG)
                .parse::<i32>().expect(ERR_MSG);
            let width = args.pop_front().expect(ERR_MSG)
                .parse::<u32>().expect(ERR_MSG);
            let height = args.pop_front().expect(ERR_MSG)
                .parse::<u32>().expect(ERR_MSG);
            let side = args.pop_front().expect(ERR_MSG)
                .parse::<u8>().expect(ERR_MSG);
            
            ghost(pos_x, pos_y, width as i32, height as i32, side);
            return Ok(());
        }
    }


    main_app();

    Ok(())

}

fn ghost(pos_x: i32, pos_y: i32, width: i32, height: i32, side: u8) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let (mx, my) = Enigo::new(&Settings::default()).unwrap().location().unwrap();


    let mut window = video_subsystem.window("_idnh_window_ghost", 1, 1)
        .opengl()
        .resizable()
        .borderless()
        .position(mx, my)
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    /* GL DEPENDENT STUFF MUST HAPPEN AFTER THIS POINT */

    window.set_opacity(0.5).unwrap();
    video_subsystem.gl_set_swap_interval(0).unwrap();
    
    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let start = Instant::now();

    let mut target: (i32, i32, i32, i32) = (0, 0, 1, 1);

    if side == 0 {
        // start from top at whatever point the mouse is at, go to full screen with 10px margin
        target = (pos_x, pos_y, width, height);
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

    target = (target.0 + 10, target.1 + 10, target.2 - 20, target.3 - 20);

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }

        let dx = crate::easing::delta(0.0, start.elapsed().as_secs_f32(), crate::easing::ease_in_out_quart(0.3));

        let rect = crate::easing::lerp4i((mx, my, 1, 1), target, dx);

        // println!("Set rect: {:?}", rect);

        window.set_position(sdl2::video::WindowPos::Positioned(rect.0), sdl2::video::WindowPos::Positioned(rect.1));
        window.set_size(rect.2 as u32, rect.3 as u32).unwrap();

        unsafe {
            gl::Viewport(0, 0, rect.2, rect.3);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        window.gl_swap_window();

    }

}

fn main_app() {

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    let (bgr, bgg, bgb, bga) = SETTINGS!(bg dark 4 f32);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window_width:  u32 = 1080;
    let window_height: u32 = 720;

    let mut window = video_subsystem.window("Insert Dungeon Name Here", window_width, window_height)
        .opengl()
        .resizable()
        .borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    /* GL DEPENDENT STUFF MUST HAPPEN AFTER THIS POINT */

    let monitors = monitor_info::get_info();

    window.set_icon(sdl2::surface::Surface::from_file("assets/textures/idnh_icon.png").unwrap());


    let _ = window.set_minimum_size(960, 540);

    video_subsystem.gl_set_swap_interval(0).unwrap();

    let char_atlas = CharAtlas::new("assets/fonts/PTMono-Regular.ttf");

    let shader = Shaders::new();

    let mut app = App::new(shader, char_atlas, window_width, window_height, &mut window, monitors);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let device_state = DeviceState::new();

    'mainloop: loop {

        if app.should_quit {
            break 'mainloop
        }

        app.clear_events();

        app.mouse.left_down = false;
        app.mouse.middle_down = false;
        app.mouse.right_down = false;
        app.mouse.left_up = false;
        app.keyboard.newly_pressed_keys.clear();
        app.keyboard.released_keys.clear();
        app.keyboard.triggered_keys.clear();

        let keys = device_state.get_keys();
        app.keyboard.capslock = keys.contains(&device_query::Keycode::CapsLock);

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'mainloop;
                }
                sdl2::event::Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } => {
                    if mouse_btn == MouseButton::Left {
                        app.mouse.left_down = true;
                        app.mouse.left_held = true;
                    }
                    else if mouse_btn == MouseButton::Right {
                        app.mouse.right_down = true;
                        app.mouse.right_held = true;
                    }
                    else if mouse_btn == MouseButton::Middle {
                        app.mouse.middle_down = true;
                        app.mouse.middle_held = true;
                    }
                }
                sdl2::event::Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } => {
                    if mouse_btn == MouseButton::Left {
                        app.mouse.left_up = true;
                        app.mouse.left_held = false;
                    }
                    else if mouse_btn == MouseButton::Right {
                        app.mouse.right_up = true;
                        app.mouse.right_held = false;
                    }
                    else if mouse_btn == MouseButton::Middle {
                        app.mouse.middle_up = true;
                        app.mouse.middle_held = false;
                    }
                }
                sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat } => {
                    
                    if scancode.is_some() {
                        let key = scancode.unwrap().name().to_string();

                        if repeat {
                            app.keyboard.triggered_keys.push(key);
                        }
                        else {
                            if key == "Left Control" {
                                app.keyboard.lctrl_held = true;
                                app.keyboard.ctrl_held = true;
                            }
                            else if key == "Right Control" {
                                app.keyboard.rctrl_held = true;
                                app.keyboard.ctrl_held = true;

                            }
                            else if key == "Left Shift" {
                                app.keyboard.lshift_held = true;
                                app.keyboard.shift_held = true;
                            }
                            else if key == "Right Shift" {
                                app.keyboard.rshift_held = true;
                                app.keyboard.shift_held = true;
                            }
                            else if key == "Left Alt" {
                                app.keyboard.lalt_held = true;
                                app.keyboard.alt_held = true;

                            }
                            else if key == "Right Alt" {
                                app.keyboard.ralt_held = true;
                                app.keyboard.alt_held = true;
                            }

                            app.keyboard.triggered_keys.push(key.clone());
                            app.keybinds.push_key(&key);
                            app.keyboard.newly_pressed_keys.push(key);
                            
                        }

                    }
                }
                sdl2::event::Event::KeyUp { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    
                    if scancode.is_some() {
                        let key = scancode.unwrap().name().to_string();

                        if key == "Left Control" {
                            app.keyboard.lctrl_held = false;
                            app.keyboard.ctrl_held = app.keyboard.rctrl_held;
                        }
                        else if key == "Right Control" {
                            app.keyboard.rctrl_held = false;
                            app.keyboard.ctrl_held = app.keyboard.lctrl_held;

                        }
                        else if key == "Left Shift" {
                            app.keyboard.lshift_held = false;
                            app.keyboard.shift_held = app.keyboard.rshift_held;
                        }
                        else if key == "Right Shift" {
                            app.keyboard.rshift_held = false;
                            app.keyboard.shift_held = app.keyboard.lshift_held;
                        }
                        else if key == "Left Alt" {
                            app.keyboard.lalt_held = false;
                            app.keyboard.alt_held = app.keyboard.ralt_held;

                        }
                        else if key == "Right Alt" {
                            app.keyboard.ralt_held = false;
                            app.keyboard.alt_held = app.keyboard.lalt_held;
                        }

                        app.keybinds.pop_key(&key);
                        app.keyboard.released_keys.push(key);

                    }

                }
                sdl2::event::Event::MouseWheel { timestamp: _, window_id: _, which: _, x, y, direction, precise_x: _, precise_y: _, mouse_x: _, mouse_y: _ } => {
                    let mut sx = x;
                    let mut sy = y;

                    if direction == MouseWheelDirection::Flipped {
                        sx = -sx;
                        sy = -sy;
                    }

                    if app.keyboard.shift_held {
                        let t = sx;
                        sx = sy;
                        sy = t;
                    }

                    app.mouse.scroll_x = sx;
                    app.mouse.scroll_y = sy;

                }
                sdl2::event::Event::Window {timestamp: _, window_id: _, win_event} => {
                    match win_event {
                        WindowEvent::FocusGained => {
                            app.window.restore();
                            app.window.raise();
                        }
                        _ => {}
                    }
                }
                _ => {
                }
            }
            app.events.push(event);
        }
        
        if app.window.is_always_on_top() {
            if device_state.get_mouse().button_pressed[1] {
                if !app.mouse.left_held {
                    app.mouse.left_down = true;
                }
                app.mouse.left_held = true;
            } else {
                if app.mouse.left_held {
                    app.mouse.left_up = true;
                }
                app.mouse.left_held = false;
            }

            if device_state.get_mouse().button_pressed[3] {
                if !app.mouse.right_held {
                    app.mouse.right_down = true;
                }
                app.mouse.right_held = true;
            } else {
                if app.mouse.right_held {
                    app.mouse.right_up = true;
                }
                app.mouse.right_held = false;
            }
        }

        let (window_x, window_y) = app.window.position();
        let (window_width, window_height) = app.window.size();
    
        app.window_pos = (window_x, window_y);
        app.window_size = (window_width, window_height);

        // app.mouse.position = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        app.mouse.position = app.enigo.location().unwrap();
        app.mouse.position = ( app.mouse.position.0 - app.window_pos.0, app.mouse.position.1 - app.window_pos.1 );
        

        app.camera.project(window_width, window_height);

        app.camera.push();

        unsafe {
            gl::ClearColor(bgr, bgg, bgb, bga);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0, 0, window_width.try_into().unwrap(), window_height.try_into().unwrap());
        }

        app.update();

        app.camera.pop();

        app.window.gl_swap_window();
    }
}
