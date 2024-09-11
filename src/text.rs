use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};

// TODO: create a font sheet either dynamically or manually, and use it for rendering text
// also make a new shader for text I guess (to apply color)


pub struct Text {
    pub position: (i32, i32),
    pub size: (u32, u32),
    content: String,
    vao: u32
}

impl Text {

    pub fn new(x: i32, y: i32, max_width: Option<u32>, max_height: Option<u32>, font_src: &str, z_index: f32) -> Text {
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

        Text {
            position: (x, y),
            size: (0, 0),
            content: "".to_string(),
            vao

        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }


}

