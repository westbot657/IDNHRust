
use std::{ffi::CString, fs};
use gl::types::*;


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
    pub text_program: u32
}

impl Shaders {
    pub fn new() -> Shaders {

        let vert1 = compile_shader(&fs::read_to_string("assets/shaders/textured_vertex_shader.vert").unwrap(), gl::VERTEX_SHADER);
        let frag1 = compile_shader(&fs::read_to_string("assets/shaders/textured_fragment_shader.frag").unwrap(), gl::FRAGMENT_SHADER);

        let vert2 = compile_shader(&fs::read_to_string("assets/shaders/color_vertex_shader.vert").unwrap(), gl::VERTEX_SHADER);
        let frag2 = compile_shader(&fs::read_to_string("assets/shaders/color_fragment_shader.frag").unwrap(), gl::FRAGMENT_SHADER);

        let vert_txt = compile_shader(&fs::read_to_string("assets/shaders/text_vertex_shader.vert").unwrap(), gl::VERTEX_SHADER);
        let frag_txt = compile_shader(&fs::read_to_string("assets/shaders/text_fragment_shader.frag").unwrap(), gl::FRAGMENT_SHADER);

        Shaders {
            textured_program: link_program(vert1, frag1),
            colored_program: link_program(vert2, frag2),
            text_program: link_program(vert_txt, frag_txt)
        }
    }

}

