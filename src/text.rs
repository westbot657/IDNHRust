use std::{collections::HashMap, panic};

use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};
use rect_packer::{Config, Packer};
use rusttype::{point, Font, Glyph, Scale};

// TODO: create a font sheet either dynamically or manually, and use it for rendering text
// also make a new shader for text I guess (to apply color)


pub struct CharAtlas {
    chars: HashMap<String, (u32, u32, u32, u32)>,
    atlas_id: u32
}

impl CharAtlas {
    pub fn new(font_path: &str) -> Self {

        const WIDTH:  u32 = 4092;
        const HEIGHT: u32 = 4092;

        let config = Config {
            width: WIDTH as i32,
            height: HEIGHT as i32,

            border_padding: 1,
            rectangle_padding: 1
        };
        let mut packer = Packer::new(config);
        let mut surf = sdl2::surface::Surface::new(WIDTH, HEIGHT, sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();


        let mut chars: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();

        let data = std::fs::read(font_path).unwrap();

        let font = Font::try_from_vec(data).unwrap_or_else(|| {
            panic!("Error loading font data from '{:?}'", font_path);
        });

        let height: f32 = 72.0;
        let pixel_height = height.ceil() as usize;

        let scale = Scale {
            x: height * 2.0,
            y: height
        };

        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        

        for codepoint in 0..=std::char::MAX as u32 {
            if let Some(character) = char::from_u32(codepoint) {
                let result = panic::catch_unwind(|| font.glyph(character));
                if result.is_err() {
                    continue;
                }

                let glyph = result.unwrap();

                

            }
        }


        Self {
            chars,
            atlas_id: 0
        }
    }
}


pub struct Text {
    pub position: (i32, i32),
    content: String,
    pub max_width: u32,
    size: (u32, u32)
}

impl Text {

}

// pub struct Text {
//     pub position: (i32, i32),
//     pub size: (u32, u32),
//     content: String,
//     vao: u32
// }

// impl Text {

//     pub fn new(x: i32, y: i32, max_width: Option<u32>, max_height: Option<u32>, font_src: &str, z_index: f32) -> Text {
//         const lower_bound: f32 = -0.5;
//         const upper_bound: f32 = 0.5;
//         let vertices: [f32; 30] = [
//             // Positions          // Texture Coords
//             lower_bound, lower_bound, z_index,      0.0, 1.0,  // Top-left
//             lower_bound, upper_bound, z_index,      0.0, 0.0,  // Bottom-left
//             upper_bound, upper_bound, z_index,      1.0, 0.0,  // Bottom-right
//             lower_bound, lower_bound, z_index,      0.0, 1.0,  // Top-left
//             upper_bound, upper_bound, z_index,      1.0, 0.0,  // Bottom-right
//             upper_bound, lower_bound, z_index,      1.0, 1.0
//         ];

//         let mut vao: GLuint = 0;
//         let mut vbo: GLuint = 0;



//         unsafe {
//             gl::GenVertexArrays(1, &mut vao);
//             gl::GenBuffers(1, &mut vbo);
        
//             gl::BindVertexArray(vao);
//             gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//             gl::BufferData(
//                 gl::ARRAY_BUFFER,
//                 (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
//                 vertices.as_ptr() as *const GLvoid,
//                 gl::STATIC_DRAW,
//             );
        
//             // Position attribute (location 0)
//             gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
//             gl::EnableVertexAttribArray(0);
        
//             // Texture Coord attribute (location 1)
//             gl::VertexAttribPointer(
//                 1,
//                 2,
//                 gl::FLOAT,
//                 gl::FALSE,
//                 5 * std::mem::size_of::<GLfloat>() as GLsizei,
//                 (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
//             );
//             gl::EnableVertexAttribArray(1);
//         }

//         Text {
//             position: (x, y),
//             size: (0, 0),
//             content: "".to_string(),
//             vao

//         }
//     }

//     pub fn set_content(&mut self, content: String) {
//         self.content = content;
//     }


// }

