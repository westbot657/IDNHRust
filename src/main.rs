extern crate sdl2;
extern crate gl;

use sdl2::{video::GLProfile, image::LoadSurface};
use std::ffi::CString;
use gl::types::*;

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(ty) };
    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            panic!(
                "{}",
                std::str::from_utf8(&buffer)
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            panic!(
                "{}",
                std::str::from_utf8(&buffer)
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}

fn load_texture(path: &str) -> GLuint {
    let surface = sdl2::surface::Surface::from_file(path).unwrap();
    let mut texture: GLuint = 0;
    
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Specify texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load image data into OpenGL texture
        let format = if surface.pixel_format_enum() == sdl2::pixels::PixelFormatEnum::RGB24 {
            gl::RGB
        } else {
            gl::RGBA
        };
        
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            surface.width() as i32,
            surface.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            surface.without_lock().unwrap().as_ptr() as *const _,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    texture
}


// Helper functions for shader compilation and linking (same as before)

struct Square {
    position: (f32, f32),    // (x, y)
    size: f32,               // side length of the square
    color: (u8, u8, u8),     // RGB color as (R, G, B)
    vao: GLuint,             // Vertex Array Object for the square
    texture: Option<GLuint>, // Texture
}

impl Square {
    fn new(size: f32, color: (u8, u8, u8), texture_path: Option<&str>) -> Square {
        let vertices: [f32; 18] = [
            -0.5,  0.5, 0.0, // Top-left
            -0.5, -0.5, 0.0, // Bottom-left
             0.5, -0.5, 0.0, // Bottom-right
            -0.5,  0.5, 0.0, // Top-left
             0.5, -0.5, 0.0, // Bottom-right
             0.5,  0.5, 0.0, // Top-right
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

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }
        
        // Load texture if a path is provided
        let texture = texture_path.map(|path| load_texture(path));

        Square {
            position: (0.0, 0.0),
            size,
            color,
            vao,
            texture,
        }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    fn render(&self, shader_program: GLuint) {
        unsafe {
            gl::UseProgram(shader_program);

            // Set position transformation (same as before)
            let col = CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, col.as_ptr());
            let transform: [f32; 16] = [
                self.size, 0.0,      0.0, 0.0,
                0.0,      self.size, 0.0, 0.0,
                0.0,      0.0,      1.0, 0.0,
                self.position.0, self.position.1, 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());

            // Set the color
            let col = CString::new("ourColor").unwrap();
            let color_loc = gl::GetUniformLocation(shader_program, col.as_ptr());
            let (r, g, b) = self.color;
            gl::Uniform3f(color_loc, r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

            // Bind the texture if available
            if let Some(texture) = self.texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture);
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

    let window = video_subsystem.window("Insert Dungeon Name Here", 800, 600)
        .opengl()
        .resizable()
        .borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Compile shaders as before
    let vert_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        uniform mat4 transform;
        void main() {
            gl_Position = transform * vec4(aPos, 1.0);
        }
    "#;

    let frag_shader_src = r#"
        #version 330 core
        out vec4 FragColor;

        uniform vec3 ourColor;
        uniform sampler2D ourTexture; // Texture input

        void main() {
            vec4 texColor = texture(ourTexture, gl_FragCoord.xy);
            FragColor = texColor; // Output the texture color
        }
    "#;

    let vert_shader = compile_shader(vert_shader_src, gl::VERTEX_SHADER);
    let frag_shader = compile_shader(frag_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vert_shader, frag_shader);

    // Create some squares
    let mut square1 = Square::new(0.2, (0, 200, 30), None);
    square1.set_position(-0.5, -0.5);

    let mut square2 = Square::new(0.3, (200, 200, 40), Some("assets/attribute_panels_concept.png"));
    square2.set_position(0.5, 0.5);

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Render the squares
            square1.render(shader_program);
            square2.render(shader_program);
        }

        window.gl_swap_window();
    }
}
