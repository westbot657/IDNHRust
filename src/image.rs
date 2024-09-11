use std::ffi::CString;

use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};

use crate::{app::App, component::Component};



pub struct Image {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub src: String,
    pub uv: (u32, u32, u32, u32),
    pub texture_size: (u32, u32),
    vao: u32
}

impl Image {
    pub fn new(x: i32, y: i32, width: u32, height: u32, src: String, uv: (u32, u32, u32, u32), texture_size: (u32, u32), z_index: f32) -> Image {
        const lower_bound: f32 = -0.5;
        const upper_bound: f32 = 0.5;
        let vertices: [f32; 30] = [
            // Positions          // Texture Coords
            lower_bound, lower_bound, z_index,      0.0, 1.0,  // Top-left
            lower_bound, upper_bound, z_index,      0.0, 0.0,  // Bottom-left
            upper_bound, upper_bound, z_index,      1.0, 0.0,  // Bottom-right
            lower_bound, lower_bound, z_index,      0.0, 1.0,  // Top-left
            upper_bound, upper_bound, z_index,      1.0, 0.0,  // Bottom-right
            upper_bound, lower_bound, z_index,      1.0, 1.0
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

        Image {
            position: (x, y),
            size: (width, height),
            src,
            uv,
            texture_size,
            vao
        }
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn render(&self, app: &App) {
        let shader_program = app.shaders.textured_program;
        let pos = app.mapCoords(&self.position);
        let sz = app.mapSize(&self.size);
        unsafe {
            gl::UseProgram(shader_program);

            // Set position transformation (same as before)
            let col = CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, col.as_ptr());
            
            let transform: [f32; 16] = [
                sz.0,             0.0,              0.0, 0.0,
                0.0,              sz.1,             0.0, 0.0,
                0.0,              0.0,              1.0, 0.0,
                pos.0+(sz.0/2.0), pos.1-(sz.1/2.0), 0.0, 1.0,
            ];
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());

            let (atlas_id, mut rect) = app.tex_atlas.get_atlas_and_rect(&self.src).unwrap();

            let uv_str = CString::new("uv").unwrap();
            let uv_loc = gl::GetUniformLocation(shader_program, uv_str.as_ptr());

            gl::Uniform4f(uv_loc,
                (rect.0 + self.uv.0) as f32 / 4096.0,
                (rect.1 + self.uv.1) as f32 / 4096.0,
                (self.uv.2) as f32 / 4096.0,
                (self.uv.3) as f32 / 4096.0
            );


            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, atlas_id);

            // let uv = CString::new("uv").unwrap();
            // let uv_pos = gl::GetUniformLocation(shader_program, uv.as_ptr());
            // gl::Uniform4f(uv_pos, self.uv.0, self.uv.1, self.uv.2, self.uv.3);

            // // Bind the texture if available
            // if let Some(texture) = self.texture {
            //     gl::ActiveTexture(gl::TEXTURE0);
            //     gl::BindTexture(gl::TEXTURE_2D, texture.0);
            // }

            // Render the square
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl Component for Image {
    fn update(&self, app: &mut crate::app::App) {
        self.render(app);
    }

    fn destroy(self) {
    }
}