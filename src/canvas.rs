use std::ffi::CString;

use cgmath::Matrix;
use enigo::Mouse;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};

use crate::{app::App, component::Component};



pub struct Canvas {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub zoom: f32,
    pub grid_spacing: u32,
    pub rotation: f32,
    pub scroll_offset: (i64, i64),
    pub children: Vec<Box<dyn Component>>,
    pub color: (u8, u8, u8, u8),
    vao: u32
}

impl Canvas {
    pub fn new(x: i32, y: i32, w: u32, h: u32, grid_spacing: u32, z_index: f32, color: (u8, u8, u8, u8)) -> Self {

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
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        
            // Position attribute (location 0)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        
            // Color attribute (location 1)
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<GLfloat>() as GLsizei,
                (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);
        }

        Self {
            position: (x, y),
            size: (w, h),
            zoom: 1.0,
            grid_spacing,
            rotation: 0.0,
            scroll_offset: (0, 0),
            children: Vec::new(),
            vao,
            color
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.size = (w, h);
    }


    pub fn render(&self, app: &App) {
        let shader_program = app.shaders.grid_shader;
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
            let (mat4, viewport) = app.camera.peek();

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

            let offset_str = CString::new("offset").unwrap();
            let offset_loc = gl::GetUniformLocation(shader_program, offset_str.as_ptr());
            // let offset = app.map_coordsi64(&self.scroll_offset);
            gl::Uniform2f(offset_loc,
                self.scroll_offset.0 as f32 / app.window_size.0 as f32,
                self.scroll_offset.1 as f32 / app.window_size.1 as f32
            );

            let zoom_str = CString::new("zoom").unwrap();
            let zoom_loc = gl::GetUniformLocation(shader_program, zoom_str.as_ptr());
            gl::Uniform1f(zoom_loc, self.zoom);

            let rot_str = CString::new("rotation").unwrap();
            let rot_loc = gl::GetUniformLocation(shader_program, rot_str.as_ptr());
            gl::Uniform1f(rot_loc, self.rotation);

            let spacing_str = CString::new("spacing").unwrap();
            let spacing_loc = gl::GetUniformLocation(shader_program, spacing_str.as_ptr());
            gl::Uniform1f(spacing_loc, self.grid_spacing as f32);


            let clr = CString::new("color").unwrap();
            let color_loc = gl::GetUniformLocation(shader_program, clr.as_ptr());
            gl::Uniform4f(color_loc, self.color.0 as f32 / 255.0, self.color.1 as f32 / 255.0, self.color.2 as f32 / 255.0, self.color.3 as f32 / 255.0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }


}

impl Component for Canvas {
    fn update(&mut self, app: &mut crate::app::App) {
        self.render(app);
    }

    fn destroy(self) {
    }
}

