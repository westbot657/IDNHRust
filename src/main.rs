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

use app::App;
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
        //.borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let shader = Shaders::new();

    let mut app = App::new(shader, window_width, window_height);


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

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0, 0, window_width.try_into().unwrap(), window_height.try_into().unwrap());
        }
            
        app.update();

        window.gl_swap_window();
    }
}
