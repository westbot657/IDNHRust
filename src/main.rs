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

use app::App;
use macros::SETTINGS;
use shaders::Shaders;
use sdl2::{event::WindowEvent, mouse::MouseButton, video::GLProfile};
use text::CharAtlas;



fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    let (bgr, bgg, bgb, bga) = SETTINGS!(bg dark 4 f32);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    

    let window_width: u32 = 1080;
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

    video_subsystem.gl_set_swap_interval(0).unwrap();

    // This atlas takes like 10 seconds to load...
    let char_atlas = CharAtlas::new("assets/fonts/PTMono-Regular.ttf");

    let shader = Shaders::new();

    let mut app = App::new(shader, char_atlas, window_width, window_height, &mut window);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'mainloop: loop {

        if app.should_quit {
            break 'mainloop
        }

        app.clear_events();

        app.mouse.left_down = false;
        app.mouse.middle_down = false;
        app.mouse.right_down = false;
        app.mouse.left_up = false;
        app.mouse.middle_up = false;
        app.mouse.right_up = false;

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

        let (window_x, window_y) = app.window.position();
        let (window_width, window_height) = app.window.size();
    
        app.window_pos = (window_x, window_y);
        app.window_size = (window_width, window_height);

        app.mouse.position = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        

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
