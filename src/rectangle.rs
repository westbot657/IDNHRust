use std::ffi::CString;

use cgmath::Matrix;
use enigo::Mouse;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};

use crate::{app::App, component::Component};


pub struct Rectangle {
    pub position: (i32, i32),
    pub size: (u32, u32),
    color: (u8, u8, u8, u8),
    vao: GLuint,
    shader: Option<u32>
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8), z_index: f32) -> Rectangle {
        const LOWER_BOUND: f32 = -0.5;
        const UPPER_BOUND: f32 = 0.5;
        let vertices: [f32; 30] = [
            LOWER_BOUND, LOWER_BOUND, z_index,      0.0, 1.0,
            LOWER_BOUND, UPPER_BOUND, z_index,      0.0, 0.0,
            UPPER_BOUND, UPPER_BOUND, z_index,      1.0, 0.0,
            LOWER_BOUND, LOWER_BOUND, z_index,      0.0, 1.0,
            UPPER_BOUND, UPPER_BOUND, z_index,      1.0, 0.0,
            UPPER_BOUND, LOWER_BOUND, z_index,      1.0, 1.0
        ];

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
        
            // Color attribute (location 1)
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                5 * size_of::<GLfloat>() as GLsizei,
                (3 * size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);
        }

        Rectangle {
            position: (x, y),
            size: (width, height),
            color,
            vao,
            shader: None
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }

    pub fn with_shader(mut self, shader: u32) -> Self {
        self.shader = Some(shader);

        self
    }

    fn render(&self, app: &App) {
        let shader_program = self.shader.or(Some(app.shaders.colored_program)).unwrap();
        let pos = app.map_coords(&self.position);
        let sz = app.map_size(&self.size);
        unsafe {
            gl::UseProgram(shader_program);

            let col = CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, col.as_ptr());
            let transform: [f32; 16] = [
                sz.0*2.0,     0.0,      0.0, 0.0,
                0.0,      sz.1*2.0,     0.0, 0.0,
                0.0,      0.0,      1.0, 0.0,
                pos.0+(sz.0), pos.1-(sz.1), 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());


            let cam_str = CString::new("camera").unwrap();
            let cam_loc = gl::GetUniformLocation(shader_program, cam_str.as_ptr());

            let view_str = CString::new("viewport").unwrap();
            let view_loc = gl::GetUniformLocation(shader_program, view_str.as_ptr());
            let (mat4, viewport, _) = app.camera.peek();

            gl::UniformMatrix4fv(cam_loc, 1, gl::FALSE, mat4.as_ptr());
            gl::Uniform4f(view_loc, 
                viewport.0 as f32 / app.window_size.0 as f32 - 1.0,
                1.0 - (viewport.1 as f32 / app.window_size.1 as f32) - (viewport.3 as f32 / app.window_size.1 as f32 * 2.0),
                viewport.2 as f32 / app.window_size.0 as f32 * 2.0,
                viewport.3 as f32 / app.window_size.1 as f32 * 2.0
            );

            let mpos_str = CString::new("mouse").unwrap();
            let mpos_loc = gl::GetUniformLocation(shader_program, mpos_str.as_ptr());
            let mpos = app.enigo.location().unwrap();
            gl::Uniform2f(mpos_loc,
                ((mpos.0 - app.window_pos.0) as f32 * 2.0 / app.window_size.0 as f32) - 1.0,
                -((mpos.1 - app.window_pos.1) as f32 * 2.0 / app.window_size.1 as f32) + 1.0
            );

            let res_str = CString::new("screen_size").unwrap();
            let res_pos = gl::GetUniformLocation(shader_program, res_str.as_ptr());
            gl::Uniform2f(res_pos,
                app.window_size.0 as f32,
                app.window_size.1 as f32
            );

            let clr = CString::new("color").unwrap();
            let color_loc = gl::GetUniformLocation(shader_program, clr.as_ptr());
            gl::Uniform4f(color_loc, self.color.0 as f32 / 255.0, self.color.1 as f32 / 255.0, self.color.2 as f32 / 255.0, self.color.3 as f32 / 255.0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

}

impl Component for Rectangle {
    fn update(&mut self, app: &mut App) {
        self.render(app);
    }

    fn destroy(self) {
    }
}

