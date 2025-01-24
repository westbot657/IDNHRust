use std::ffi::CString;

use cgmath::{Matrix, Matrix4};
use enigo::Mouse;
use gl::types::GLuint;

use crate::{app::App, component::Component};
use crate::component::setup_gl_pos;

pub struct Rectangle {
    pub position: (i32, i32),
    pub size: (u32, u32),
    color: (u8, u8, u8, u8),
    vao: GLuint,
    shader: Option<u32>,
    pub uid: String
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8), z_index: f32) -> Rectangle {
        const LOWER_BOUND: f32 = -0.5;
        const UPPER_BOUND: f32 = 0.5;
        let vertices: [f32; 18] = [
            LOWER_BOUND, LOWER_BOUND, z_index,
            LOWER_BOUND, UPPER_BOUND, z_index,
            UPPER_BOUND, UPPER_BOUND, z_index,
            LOWER_BOUND, LOWER_BOUND, z_index,
            UPPER_BOUND, UPPER_BOUND, z_index,
            UPPER_BOUND, LOWER_BOUND, z_index
        ];

        let vao = setup_gl_pos(vertices);

        Rectangle {
            position: (x, y),
            size: (width, height),
            color,
            vao,
            shader: None,
            uid: "".to_string() // Most rectangles don't need this so it can be set after creation
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
        let shader_program = self.shader.unwrap_or(app.shaders.colored_program);
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
                (pos.0+sz.0), pos.1-sz.1, 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());


            let cam_str = CString::new("camera").unwrap();
            let cam_loc = gl::GetUniformLocation(shader_program, cam_str.as_ptr());

            let view_str = CString::new("viewport").unwrap();
            let view_loc = gl::GetUniformLocation(shader_program, view_str.as_ptr());
            let (translation_matrix, normal_matrix, _, viewport) = app.camera.peek();

            gl::UniformMatrix4fv(cam_loc, 1, gl::FALSE, translation_matrix.as_ptr());
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

}

