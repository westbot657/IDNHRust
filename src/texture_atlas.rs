extern crate glob;

use std::collections::HashMap;

use gl::types::GLuint;
use glob::glob;
use rect_packer::{Config, Packer};
use sdl2::{image::LoadSurface, rect::Rect, surface::Surface};
use crate::macros::CONST;



pub fn convert_tex_to_gl(surface: &Surface, mipmap_style: u8) -> (GLuint, (u32, u32)) {
    let mut texture: GLuint = 0;
    
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        if mipmap_style == 0 {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        else if mipmap_style == 1 {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        }


        let format = if surface.pixel_format_enum() == sdl2::pixels::PixelFormatEnum::RGB24 {
            gl::RGB
        } else {
            gl::RGBA
        };
        
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            surface.width() as i32,
            surface.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            surface.without_lock().unwrap().as_ptr() as *const _,
        );

        if mipmap_style == 0 {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    (texture, surface.size())
}

pub struct TextureAtlas<'a> {
    pub position_data: HashMap<String, (u32, u32, u32, u32)>,
    pub atlas_map: HashMap<String, u32>,
    pub textures: Vec<Surface<'a>>,
    pub idx_to_gluint: HashMap<u32, u32>
}

impl<'a> TextureAtlas<'a> {
    pub fn new() -> TextureAtlas<'a> {

        const WIDTH: u32 = CONST!(atlas);
        const HEIGHT: u32 = CONST!(atlas);

        let config = Config {
            width: WIDTH as i32,
            height: HEIGHT as i32,

            border_padding: 1,
            rectangle_padding: 1
        };

        let mut atlases: Vec<Packer> = Vec::new();
        let mut packer = Packer::new(config);
        let mut surf = Surface::new(WIDTH, HEIGHT, sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();

        // this indicates a file's position on an atlas
        let mut position_data: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();

        // this indicates which atlas a file's texture is on
        let mut atlas_map: HashMap<String, u32> = HashMap::new();

        let mut surfaces: Vec<Surface> = Vec::new();

        for texture in glob("assets/textures/**/*.png").expect("Failed to read glob pattern") {
            let path = texture.unwrap(); // if a texture fails to load, then panic
            

            let surface = Surface::from_file(&path).unwrap();

            let (w, h) = surface.size();

            if !packer.can_pack(w as i32, h as i32, false) {
                //atlases.push(packer);
                packer = Packer::new(config);
                surfaces.push(surf);
                surf = Surface::new(WIDTH, HEIGHT, sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
            }

            let rect = packer.pack(w as i32, h as i32, false).unwrap();
            
            surface.blit(Rect::new(0, 0, surface.size().0, surface.size().1), &mut surf, Rect::new(rect.x, rect.y, rect.width as u32, rect.height as u32)).unwrap();

            position_data.insert(path.to_str().unwrap().to_owned(), (rect.x as u32, rect.y as u32, rect.width as u32, rect.height as u32));
            atlas_map.insert(path.to_str().unwrap().to_owned(), atlases.len() as u32);
            

        }

        // surf.save(&format!("out/texture_atlas_{}.png", atlases.len()).as_str()).unwrap();

        atlases.push(packer);
        surfaces.push(surf);

        TextureAtlas {
            position_data,
            atlas_map,
            textures: surfaces,
            idx_to_gluint: HashMap::new()
        }
    }

    pub fn get_atlas_and_rect(&self, texture:&str) -> Result<(u32, (u32, u32, u32, u32)), String> {

        if self.atlas_map.contains_key(texture) {
            if self.position_data.contains_key(texture) {
                let map_idx = self.atlas_map.get(texture).unwrap().to_owned();

                Ok((self.idx_to_gluint.get(&map_idx).unwrap().to_owned(), *self.position_data.get(texture).unwrap()))
            } else {
                Err(format!("Texture '{}' not in Atlas", texture).to_string())
            }
        } else {
            Err(format!("Texture '{}' not in Atlas", texture).to_string())
        }
    }


}

