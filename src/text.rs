use std::{collections::HashMap, ffi::CString, panic};
use cgmath::Matrix;
use rect_packer::{Config, Packer};
use rusttype::{point, Font, GlyphId, Scale};
use crate::{app::App, component::Component, texture_atlas::convert_tex_to_gl, macros::CONST};
use crate::component::setup_gl_pos_tex;
use crate::es3::style_flags;
use crate::text_input_handler::IdxSize;

pub struct CharAtlas {
    chars: HashMap<String, ((u32, u32, u32, u32), i32)>,
    atlas_id: u32,
    vao: u32
}

type Bounds = (Option<i32>, Option<i32>, Option<u32>, Option<u32>);

impl CharAtlas {
    pub fn new(font_path: &str) -> Self {

        const WIDTH:  u32 = CONST!(text atlas);
        const HEIGHT: u32 = CONST!(text atlas);

        let config = Config {
            width: WIDTH as i32,
            height: HEIGHT as i32,

            border_padding: 1,
            rectangle_padding: 2
        };
        let mut packer = Packer::new(config);
        let mut surf = sdl2::surface::Surface::new(WIDTH, HEIGHT, sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();


        let mut chars: HashMap<String, ((u32, u32, u32, u32), i32)> = HashMap::new();

        let data = std::fs::read(font_path).unwrap_or_else(|_| panic!("Cannot find font {}", font_path));

        let font = Font::try_from_vec(data).unwrap_or_else(|| {
            panic!("Error loading font data from '{:?}'", font_path);
        });

        let height: f32 = CONST!(text height f32);

        let scale = Scale {
            x: height * 2.0,
            y: height
        };

        

        for codepoint in 0..=0x4E00u32 {
            if let Some(character) = char::from_u32(codepoint) {
                let result = panic::catch_unwind(|| font.glyph(character));
                if result.is_err() {
                    continue;
                }


                let glyph = result.unwrap().scaled(scale);

                if glyph.id() == GlyphId(0) {
                    continue;
                }


                let pos_glyph = glyph.positioned(point(0.0, 0.0));



                let bounds = pos_glyph.pixel_bounding_box();
                if bounds.is_none() {
                    continue;
                }

                let bounds = bounds.unwrap();

                if !packer.can_pack(bounds.width(), bounds.height(), false) {
                    panic!("Font filled the entire texture atlas!! (at character '{}')", character);
                }

                let rect = packer.pack(bounds.width(), bounds.height(), false).unwrap();

                let x = rect.x as u32;
                let y = rect.y as u32;
                chars.insert(character.to_string(), ((x, y, rect.width as u32, rect.height as u32), bounds.min.y));

                pos_glyph.draw(|dx, dy, v| {
                    surf.fill_rect(
                        Some(sdl2::rect::Rect::new(
                            (x+dx) as i32,
                            (y+dy) as i32,
                            1,
                            1)),
                        sdl2::pixels::Color {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: (v*255.0) as u8
                        }
                    ).unwrap();
                });

            }
        }

        // surf.save(&format!("out/text_atlas-{}.png", font_path.replace("/", "_")).as_str()).unwrap();


        let atlas_id = convert_tex_to_gl(&surf, 1).0;


        const LOWER_BOUND: f32 = -0.5;
        const UPPER_BOUND: f32 = 0.5;
        let vertices: [f32; 30] = [
            LOWER_BOUND, LOWER_BOUND, 0.0,      0.0, 1.0,  // Top-left
            LOWER_BOUND, UPPER_BOUND, 0.0,      0.0, 0.0,  // Bottom-left
            UPPER_BOUND, UPPER_BOUND, 0.0,      1.0, 0.0,  // Bottom-right
            LOWER_BOUND, LOWER_BOUND, 0.0,      0.0, 1.0,  // Top-left
            UPPER_BOUND, UPPER_BOUND, 0.0,      1.0, 0.0,  // Bottom-right
            UPPER_BOUND, LOWER_BOUND, 0.0,      1.0, 1.0
        ];

        let vao = setup_gl_pos_tex(vertices);


        Self {
            chars,
            atlas_id,
            vao
        }
    }

    pub fn skip_char(&self, draw_x: &mut u32, draw_y: &mut u32, character: &str, scale: f32) {
        const HEIGHT: u32 = CONST!(text height);

        if character == "\n" {
            *draw_y += ((HEIGHT as f32 * scale) + (4.0 * scale)) as u32;
            *draw_x = 0;
        }
        else if character == " " || self.chars.contains_key(character) {
            *draw_x += ((HEIGHT as f32 / 2.0 * scale) + (4.0 * scale)) as u32;
        }
    }

    pub fn render_char(&self, app: &App, x: i32, y: i32, draw_x: &mut u32, draw_y: &mut u32, character: &str, z_index: f32, scale: f32, bounds: Bounds) {
        const HEIGHT: u32 = CONST!(text height);

        if character == "\n" {
            *draw_y += ((HEIGHT as f32 * scale) + (4.0 * scale)) as u32;
            *draw_x = 0;
        }
        else if character == " " {
            *draw_x += ((HEIGHT as f32 / 2.0 * scale) + (4.0 * scale)) as u32;
        }
        else if self.chars.contains_key(character) {
            let rect = self.chars.get(character).unwrap();

            let tx = ((((HEIGHT as f32 / 2.0 * scale) + 4.0) - (rect.0.2 as f32 * scale)) / 4.0).round() as i32;

            let pos = app.map_coords(&(x+*draw_x as i32+tx, ((y+*draw_y as i32) as f32 + (rect.1 as f32 * scale) + (HEIGHT as f32 * scale)).round() as i32));

            let sz = app.map_size(&((rect.0.2 as f32 / 2.0 * scale).round() as u32, (rect.0.3 as f32 * scale).round() as u32));

            unsafe {


                let col = CString::new("transform").unwrap();
                let transform_loc = gl::GetUniformLocation(app.shaders.text_program, col.as_ptr());
                
                let transform: [f32; 16] = [
                    sz.0*2.0,     0.0,          0.0, 0.0,
                    0.0,          sz.1*2.0,     0.0, 0.0,
                    0.0,          0.0,          1.0, 0.0,
                    pos.0+(sz.0), pos.1-(sz.1), z_index, 1.0,
                ];
                gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());


                let uv_str = CString::new("uv").unwrap();
                let uv_loc = gl::GetUniformLocation(app.shaders.text_program, uv_str.as_ptr());
    
                gl::Uniform4f(uv_loc,
                    rect.0.0 as f32 / CONST!(text atlas f32),
                    rect.0.1 as f32 / CONST!(text atlas f32),
                    rect.0.2 as f32 / CONST!(text atlas f32),
                    rect.0.3 as f32 / CONST!(text atlas f32)
                );

                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                
                
                
            }

            *draw_x += ((HEIGHT as f32 / 2.0 * scale) + (4.0 * scale)) as u32;
            // *draw_x += (rect.0.2 as f32 / 2.0 * scale) as i32 + (4.0 * scale) as i32;

        }

    }

    pub fn draw_text(&self, app: &App, x: i32, y: i32, text: &str, scale: f32, bounds: Bounds, z_index: f32, color: (u8, u8, u8, u8), styles: u8) {
        let mut draw_x: u32 = 0;
        let mut draw_y: u32 = 0;

        let (bound_x, bound_y, bound_width, bound_height) = bounds;
        
        let shader_program = app.shaders.text_program;

        unsafe {
            gl::UseProgram(shader_program);

            let cam_str = CString::new("camera").unwrap();
            let cam_loc = gl::GetUniformLocation(shader_program, cam_str.as_ptr());

            let view_str = CString::new("viewport").unwrap();
            let view_loc = gl::GetUniformLocation(shader_program, view_str.as_ptr());
            let (translation_matrix, normal_matrix, _, viewport) = app.camera.peek();

            gl::UniformMatrix4fv(cam_loc, 1, gl::FALSE, translation_matrix.into().as_ptr());
            gl::Uniform4f(view_loc,
                          viewport.0 as f32 / app.window_size.0 as f32 - 1.0,
                          1.0 - (viewport.1 as f32 / app.window_size.1 as f32) - (viewport.3 as f32 / app.window_size.1 as f32 * 2.0),
                          viewport.2 as f32 / app.window_size.0 as f32 * 2.0,
                          viewport.3 as f32 / app.window_size.1 as f32 * 2.0
            );

            let col_str = CString::new("color").unwrap();
            let col_loc = gl::GetUniformLocation(shader_program, col_str.as_ptr());
            gl::Uniform4f(col_loc,
                          color.0 as f32 / 255.0,
                          color.1 as f32 / 255.0,
                          color.2 as f32 / 255.0,
                          color.3 as f32 / 255.0,
            );

            let italic_str = CString::new("italic").unwrap();
            let italic_loc = gl::GetUniformLocation(shader_program, italic_str.as_ptr());
            gl::Uniform1f(italic_loc, (styles & style_flags::ITALIC) as f32);

            let draw_str = CString::new("draw_clip").unwrap();
            let draw_loc = gl::GetUniformLocation(app.shaders.text_program, draw_str.as_ptr());
            // let (drawX, drawY) = app.map_coords(&(bound_x.unwrap_or(-5000) + x*2, bound_y.unwrap_or(-5000) + y*2));
            // let (drawW, drawH) = app.map_size(&(bound_width.unwrap_or(10000), bound_height.unwrap_or(10000)));
            // println!("Draw clip: {}, {}, {}, {}", drawX+1.0, drawY, drawW, drawH);
            // gl::Uniform4f(draw_loc,
            //               drawX + 1.0, drawY, drawW, drawH
            // );
            gl::Uniform4f(draw_loc,
                          bound_x.unwrap_or(-5000) as f32 / app.window_size.0 as f32 - 1.0,
                          1.0 - (bound_y.unwrap_or(-5000) as f32 / app.window_size.1 as f32) - (bound_height.unwrap_or(10000) as f32 / app.window_size.1 as f32 * 2.0),
                          bound_width.unwrap_or(10000) as f32 / app.window_size.0 as f32 * 2.0,
                          bound_height.unwrap_or(10000) as f32 / app.window_size.1 as f32 * 2.0
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.atlas_id);
            gl::BindVertexArray(self.vao);

        }

        for character in text.split("") {
            if character.is_empty() {continue}
            if draw_x > bound_width.unwrap_or(u32::MAX) || (draw_y as i32 + (scale * CONST!(text height f32)) as i32) < bound_y.unwrap_or(0) || (draw_x as i32 + (scale * CONST!(text height f32)) as i32) < bound_x.unwrap_or(0) {
                self.skip_char(&mut draw_x, &mut draw_y, character, scale);
            } else {
                self.render_char(app, x, y, &mut draw_x, &mut draw_y, character, z_index, scale, bounds);
            }
            if draw_y as i64 > bound_height.unwrap_or(u32::MAX) as i64 {
                return
            }
        }
    }

}


pub struct FontHandler {
    pub normal: CharAtlas,
    pub italic: CharAtlas,
    pub bold: CharAtlas,
    pub bold_italic: CharAtlas
}

impl FontHandler {
    pub fn new(normal: &str, italic: &str, bold: &str, bold_italic: &str) -> Self {

        Self {
            normal: CharAtlas::new(normal),
            italic: CharAtlas::new(italic),
            bold: CharAtlas::new(bold),
            bold_italic: CharAtlas::new(bold_italic)
        }
    }

    pub fn style(&self, style: &str) -> &CharAtlas {
        if style == "italic" {
            &self.italic
        }
        else if style == "bold" {
            &self.bold
        }
        else if style == "bold-italic" {
            &self.bold_italic
        }
        else {
            &self.normal
        }
    }

    pub fn style_flagged(&self, style: u8) -> &CharAtlas {
        if style & style_flags::ITALIC != 0 {
            if style & style_flags::BOLD != 0 {
                &self.bold_italic
            } else {
                &self.italic
            }
        } else if style & style_flags::BOLD != 0 {
            &self.bold
        } else {
            &self.normal
        }
    }

}


pub struct Text {
    pub position: (i32, i32),
    pub content: String,
    pub bounds: Bounds,
    size: (u32, u32),
    pub scale: f32,
    z_index: f32,
    color: (u8, u8, u8, u8),
    pub styles: u8,
    pub uid: String
}

impl Text {
    pub fn new(x: i32, y: i32, content: &str, bounds: Bounds, scale: f32, z_index: f32, color: (u8, u8, u8, u8)) -> Self {
        Self {
            position: (x, y),
            content: content.to_string(),
            bounds,
            size: (0, 0),
            scale,
            z_index,
            color,
            styles: 0,
            uid: "".to_string()
        }
    }

    pub fn set_styles(&mut self, styles: u8) {
        self.styles = styles;
    }
    
    pub fn get_draw_offset(&self, app: &App, index: IdxSize) -> Option<(u32, u32)> {
        // println!("Get draw of index {} (content: {:?})", index, self.content);
        if index <= self.content.chars().count() {
            let mut x: u32 = 0;
            let mut y: u32 = 0;
            for c in self.content.chars().take(index) {
                let font = app.font_handler.style_flagged(self.styles);
                font.skip_char(&mut x, &mut y, &c.to_string(), self.scale);
            }
            return Some((x, y))
        }
        
        None
    }
    
}

impl Component for Text {
    fn update(&mut self, app: &mut App) {

        app.font_handler.style_flagged(self.styles).draw_text(app, self.position.0, self.position.1, &self.content, self.scale, self.bounds, self.z_index, self.color, self.styles)
    }


    fn destroy(self) {
    }
}
