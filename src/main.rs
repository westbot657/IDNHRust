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

use app::App;
use cgmath::{Deg, Rad};
use shaders::Shaders;
use sdl2::video::GLProfile;



fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window_x = 200;
    let window_y = 200;
    let window_width: u32 = 1080;
    let window_height: u32 = 720;

    let window = video_subsystem.window("Insert Dungeon Name Here", window_width, window_height)
        .opengl()
        .resizable()
        .borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let shader = Shaders::new();

    let mut app = App::new(shader, window_width, window_height);


    // app.camera.viewport = (0, 0, window_width, window_height);
    // app.camera.set_scale(100.0, 100.0);
    // app.camera.set_position(0.0, 0.0);
    // app.camera.set_rotation(Rad(0.0));

    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'mainloop: loop {

        app.clear_events();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
            app.events.push(event);
        }

        let (window_width, window_height) = window.size();
        app.window_size = (window_width, window_height);

        app.camera.project(window_width, window_height);

        app.camera.push();

        // app.camera.set_position(
        //     event_pump.mouse_state().x() as f32 / window_width as f32,
        //     event_pump.mouse_state().y() as f32 / window_height as f32
        // );

        // app.camera.set_scale(event_pump.mouse_state().y() as f32 / window_height as f32, event_pump.mouse_state().y() as f32 / window_height as f32);

        // app.camera.set_rotation(Deg(event_pump.mouse_state().x() as f32).into());

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0, 0, window_width.try_into().unwrap(), window_height.try_into().unwrap());
        }
            
        app.update();

        app.camera.pop();

        window.gl_swap_window();
    }
}
