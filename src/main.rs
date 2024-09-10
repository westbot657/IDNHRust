extern crate sdl2;
extern crate gl;

use sdl2::{image::LoadSurface, video::GLProfile};
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

fn load_texture(path: &str) -> (GLuint, (u32, u32)) {
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

    (texture, surface.size())
}


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
            println!("{:?}", uv);
            let f_uv = (uv.0 as f32 / size.0 as f32, uv.1 as f32 / size.1 as f32, uv.2 as f32 / size.0 as f32, uv.3 as f32 / size.1 as f32);
            println!("{:?}", f_uv);
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

    fn render(&self, shader_program: GLuint, aspect_ratio: f32, screen_width: u32, screen_height: u32) {
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
    let window_width: u32 = 800;
    let window_height: u32 = 600;

    let window = video_subsystem.window("Insert Dungeon Name Here", window_width, window_height)
        .opengl()
        .resizable()
        //.borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Compile shaders as before
    let vert_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;

        out vec2 TexCoord;

        uniform mat4 transform;

        void main() {
            gl_Position = transform * vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }
    "#;

    let frag_shader_src = r#"
        #version 330 core
        out vec4 FragColor;

        in vec2 TexCoord;

        uniform sampler2D ourTexture;
        uniform vec4 uv;

        void main() {
            vec4 texColor = texture(ourTexture, TexCoord);
            if (texColor.a < 0.1)
                discard;
            
            if (!(uv.x <= TexCoord.x && TexCoord.x <= uv.x + uv.z && uv.y <= TexCoord.y && TexCoord.y <= uv.y + uv.w)) {
                discard;
            } else {
                FragColor = texColor;
            }
        }
    "#;

    let vert_shader = compile_shader(vert_shader_src, gl::VERTEX_SHADER);
    let frag_shader = compile_shader(frag_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vert_shader, frag_shader);

    // Create some squares
    let mut square1 = Square::new((0, 0), Some("assets/test.png"), None);
    square1.set_position(-0.5, -0.5);

    let mut square2 = Square::new((0, 0), Some("assets/test5.png"), Some((0, 0, 18, 18)));
    square2.set_position(0.0, 0.0);

    let mut square3 = Square::new((0, 0), Some("assets/test2.png"), None);
    square3.set_position(0.5, -0.5);


    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }

        let (window_width, window_height) = window.size();

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            let aspect_ratio = window_width as f32 / window_height as f32;
            gl::Viewport(0, 0, window_width.try_into().unwrap(), window_height.try_into().unwrap());

            // Render the squares
            square1.render(shader_program, aspect_ratio, window_width, window_height);
            square2.render(shader_program, aspect_ratio, window_width, window_height);
            square3.render(shader_program, aspect_ratio, window_width, window_height);
        }

        window.gl_swap_window();
    }
}
