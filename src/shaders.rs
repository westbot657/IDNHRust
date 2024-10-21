
use std::{ffi::CString, fs};
use gl::types::*;

macro_rules! shader {
    ( vert $path:expr ) => {
        compile_shader(&fs::read_to_string("assets/shaders/".to_owned() + $path).unwrap(), gl::VERTEX_SHADER)
    };
    ( frag $path:expr) => {
        compile_shader(&fs::read_to_string("assets/shaders/".to_owned() + $path).unwrap(), gl::FRAGMENT_SHADER)

    }
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
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

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
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


pub struct Shaders {
    pub textured_program: u32,
    pub colored_program: u32,
    pub text_program: u32,
    pub prox_fade: u32,
    pub prox_fade_red: u32,
    pub prox_fade_texture_white: u32,
    pub canvas_grid_shader: u32,
    pub canvas_dots_shader: u32
}

impl Shaders {
    pub fn new() -> Shaders {

        let texture_vert = shader!(vert "textured_vertex_shader.vert");
        let frag1 = shader!(frag "textured_fragment_shader.frag");

        let colored_vert = shader!(vert "color_vertex_shader.vert");
        let frag2 = shader!(frag "color_fragment_shader.frag");

        let vert_txt = shader!(vert "text_vertex_shader.vert");
        let frag_txt = shader!(frag "text_fragment_shader.frag");

        let prox_fade_frag = shader!(frag "effects/glow/prox_fade.frag");

        let prox_fade_red_frag = shader!(frag "effects/glow/prox_fade_red.frag");

        let prox_fade_texture_frag = shader!(frag "effects/glow/prox_fade_white_texture.frag");

        let canvas_frag_grid = shader!(frag "canvas_grid.frag");
        let canvas_frag_dots = shader!(frag "canvas_dots.frag");

        Shaders {
            textured_program: link_program(texture_vert, frag1),
            colored_program: link_program(colored_vert, frag2),
            text_program: link_program(vert_txt, frag_txt),
            prox_fade: link_program(colored_vert, prox_fade_frag),
            prox_fade_red: link_program(texture_vert, prox_fade_red_frag),
            prox_fade_texture_white: link_program(texture_vert, prox_fade_texture_frag),
            canvas_grid_shader: link_program(colored_vert, canvas_frag_grid),
            canvas_dots_shader: link_program(colored_vert, canvas_frag_dots)
        }
    }

}

