use std::any::Any;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};
use crate::app::App;

pub trait ComponentToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: ComponentToAny {
    fn update(&mut self, app: &mut App);

    fn destroy(self);
}

pub fn setup_gl_pos_tex(vertices: [f32; 30]) -> GLuint {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Position attribute (location 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * size_of::<GLfloat>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Tex Coord (location 1)
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<GLfloat>() as GLsizei,
            (3 * size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    vao
}

pub fn setup_gl_pos(vertices: [f32; 18]) -> GLuint {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Position attribute (location 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<GLfloat>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }
    
    vao
}
