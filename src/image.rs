use std::ffi::CString;

use cgmath::{Matrix, Matrix4};
use enigo::Mouse;

use crate::{app::App, component::Component, macros::CONST};
use crate::component::setup_gl_pos_tex;

pub struct Image {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub src: String,
    pub uv: (u32, u32, u32, u32),
    vao: u32,
    shader: Option<u32>,
    pub uid: String
}

impl Image {
    pub fn new(x: i32, y: i32, width: u32, height: u32, src: impl ToString, uv: (u32, u32, u32, u32), z_index: f32) -> Image {
        const LOWER_BOUND: f32 = -0.5;
        const UPPER_BOUND: f32 = 0.5;
        let vertices: [f32; 30] = [
            LOWER_BOUND, LOWER_BOUND, z_index,      0.0, 1.0,  // top right
            LOWER_BOUND, UPPER_BOUND, z_index,      0.0, 0.0,  // top left
            UPPER_BOUND, UPPER_BOUND, z_index,      1.0, 0.0,  // bottom left
            LOWER_BOUND, LOWER_BOUND, z_index,      0.0, 1.0,  // top right
            UPPER_BOUND, UPPER_BOUND, z_index,      1.0, 0.0,  // bottom left
            UPPER_BOUND, LOWER_BOUND, z_index,      1.0, 1.0   // bottom right
        ];

        let vao = setup_gl_pos_tex(vertices);

        Image {
            position: (x, y),
            size: (width, height),
            src: src.to_string().replace("/", "\\"),
            uv,
            vao,
            shader: None,
            uid: "".to_string()
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn with_shader(mut self, shader: u32) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }

    fn render(&self, app: &App) {
        let shader_program = self.shader.unwrap_or(app.shaders.textured_program);
        let pos = app.map_coords(&self.position);
        let sz = app.map_size(&self.size);
        unsafe {
            gl::UseProgram(shader_program);

            let transform_str = CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, transform_str.as_ptr());
            
            let transform: [f32; 16] = [
                sz.0*2.0,     0.0,          0.0, 0.0,
                0.0,          sz.1*2.0,     0.0, 0.0,
                0.0,          0.0,          1.0, 0.0,
                pos.0+(sz.0), pos.1-(sz.1), 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());

            let (atlas_id, rect) = app.tex_atlas.get_atlas_and_rect(&self.src).expect(format!("Could not load texture {:?}", self.src).as_str());

            let uv_str = CString::new("uv").unwrap();
            let uv_loc = gl::GetUniformLocation(shader_program, uv_str.as_ptr());

            gl::Uniform4f(uv_loc,
                (rect.0 + self.uv.0) as f32 / CONST!(atlas f32),
                (rect.1 + self.uv.1) as f32 / CONST!(atlas f32),
                (self.uv.2) as f32 / CONST!(atlas f32),
                (self.uv.3) as f32 / CONST!(atlas f32)
            );

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

            let res_str = CString::new("screen_size").unwrap();
            let res_pos = gl::GetUniformLocation(shader_program, res_str.as_ptr());
            gl::Uniform2f(res_pos,
                app.window_size.0 as f32,
                app.window_size.1 as f32
            );

            let mpos_str = CString::new("mouse").unwrap();
            let mpos_loc = gl::GetUniformLocation(shader_program, mpos_str.as_ptr());
            let mpos = app.enigo.location().unwrap();
            gl::Uniform2f(mpos_loc,
                ((mpos.0 - app.window_pos.0) as f32 * 2.0 / app.window_size.0 as f32) - 1.0,
                -((mpos.1 - app.window_pos.1) as f32 * 2.0 / app.window_size.1 as f32) + 1.0
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, atlas_id);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl Component for Image {
    fn update(&mut self, app: &mut App) {
        self.render(app);
    }

}
