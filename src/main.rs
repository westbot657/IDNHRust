extern crate sdl2;
extern crate gl;

mod shaders;
mod surface;
mod texture_atlas;
mod app;
mod component;
mod rectangle;
mod image;

use app::App;
use shaders::Shaders;
use sdl2::{image::LoadSurface, surface::Surface, video::GLProfile};
use texture_atlas::load_texture;
use std::ffi::CString;
use gl::types::*;

use shaders::link_program;




// Helper functions for shader compilation and linking (same as before)

struct Square {
    position: (f32, f32),    // (x, y)
    size: (u32, u32),               // side length of the square
    vao: GLuint,             // Vertex Array Object for the square
    texture: Option<(GLuint, (u32, u32))>, // Texture
    uv: (f32, f32, f32, f32)
}

impl Square {
    fn new(size: (u32, u32), texture_path: Option<&str>, uv: Option<(u32, u32, u32, u32)>) -> Square {
        let vertices: [f32; 30] = [
            // Positions          // Texture Coords
            -0.5,  -0.5, 0.0,      0.0, 1.0,  // Top-left
            -0.5, 0.5, 0.0,      0.0, 0.0,  // Bottom-left
            0.5, 0.5, 0.0,      1.0, 0.0,  // Bottom-right
            -0.5, -0.5, 0.0,      0.0, 1.0,  // Top-left
            0.5, 0.5, 0.0,      1.0, 0.0,  // Bottom-right
            0.5, -0.5, 0.0,         1.0, 1.0
        ];

        // Create a VAO and VBO for the square
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        
            // Position attribute (location 0)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        
            // Texture Coord attribute (location 1)
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<GLfloat>() as GLsizei,
                (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);
        }
        
        // Load texture if a path is provided
        let texture = texture_path.map(|path| load_texture(path));
        if texture.is_some() {
            let size = (texture.unwrap().1.0, texture.unwrap().1.1);
            let uv = uv.or(Some((0, 0, size.0, size.1))).unwrap();
            let f_uv = (uv.0 as f32 / size.0 as f32, uv.1 as f32 / size.1 as f32, uv.2 as f32 / size.0 as f32, uv.3 as f32 / size.1 as f32);
            Square {
                position: (0.0, 0.0),
                size,
                vao,
                texture,
                uv: f_uv
            }
        } else {
            let uv = uv.or(Some((0, 0, size.0, size.1))).unwrap();
            let f_uv = (uv.0 as f32 / size.0 as f32, uv.1 as f32 / size.1 as f32, uv.2 as f32 / size.0 as f32, uv.3 as f32 / size.1 as f32);
            Square {
                position: (0.0, 0.0),
                size,
                vao,
                texture,
                uv: f_uv
            }
        }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    fn render(&self, shader_program: GLuint, screen_width: u32, screen_height: u32) {
        unsafe {
            gl::UseProgram(shader_program);

            // Set position transformation (same as before)
            let col = CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, col.as_ptr());
            
            let transform: [f32; 16] = [
                self.size.0 as f32 / screen_width as f32, 0.0,      0.0, 0.0,
                0.0,      self.size.1 as f32 / screen_height as f32, 0.0, 0.0,
                0.0,      0.0,      1.0, 0.0,
                self.position.0, self.position.1, 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());


            let uv = CString::new("uv").unwrap();
            let uv_pos = gl::GetUniformLocation(shader_program, uv.as_ptr());
            gl::Uniform4f(uv_pos, self.uv.0, self.uv.1, self.uv.2, self.uv.3);

            // Bind the texture if available
            if let Some(texture) = self.texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.0);
            }

            // Render the square
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

fn main() {
    // Initialize SDL2 and OpenGL as before
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

    let mut app = App::new(shader);

    // Create some squares
    // let mut square1 = Square::new((0, 0), Some("assets/test.png"), None);
    // square1.set_position(-0.5, -0.5);

    // let mut square2 = Square::new((0, 0), Some("assets/test5.png"), Some((0, 0, 18, 18)));
    // square2.set_position(0.0, 0.0);

    // let mut square3 = Square::new((0, 0), Some("assets/test2.png"), None);
    // square3.set_position(0.5, -0.5);


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

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0, 0, window_width.try_into().unwrap(), window_height.try_into().unwrap());

            

            // Render the squares
            // square1.render(shader_program, window_width, window_height);
            // square2.render(shader_program, window_width, window_height);
            // square3.render(shader_program, window_width, window_height);
        }

        app.update();

        window.gl_swap_window();
    }
}
