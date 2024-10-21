use std::collections::VecDeque;
use std::ffi::CString;

use cgmath::{Matrix, Rad};
use enigo::Mouse;

use crate::{app::App, component::Component};
use crate::component::setup_gl_pos_tex;
use crate::macros::collides;
use crate::object_tree::ObjectTree;

pub struct Canvas {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub zoom: f32,
    pub grid_spacing: u32,
    pub rotation: f32,
    pub scroll_offset: (i64, i64),
    pub children: Vec<Box<dyn Component>>,
    pub object_tree: ObjectTree,
    pub color: (u8, u8, u8, u8),
    vao: u32,
    uid: String
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

        let vao = setup_gl_pos_tex(vertices);

        Self {
            position: (x, y),
            size: (w, h),
            zoom: 1.0,
            grid_spacing,
            rotation: 0.0,
            scroll_offset: (0, 0),
            children: Vec::new(),
            object_tree: ObjectTree::new(),
            vao,
            color,
            uid: "canvas".to_string(),
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.size = (w, h);
    }


    pub fn render(&self, app: &App) {
        let shader_program = app.shaders.canvas_dots_shader;
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
            let (mat4, viewport, camera_position) = app.camera.peek();

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
                self.scroll_offset.0 as f32 / app.window_size.1 as f32,
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

            let canvas_origin_str = CString::new("canvas_origin").unwrap();
            let canvas_origin_loc = gl::GetUniformLocation(shader_program, canvas_origin_str.as_ptr());

            let orig = app.map_coords(&(camera_position.0 + self.position.0, camera_position.1 + self.position.1));

            gl::Uniform2f(canvas_origin_loc,
                orig.0, orig.1
            );


            let clr = CString::new("color").unwrap();
            let color_loc = gl::GetUniformLocation(shader_program, clr.as_ptr());
            gl::Uniform4f(color_loc, self.color.0 as f32 / 255.0, self.color.1 as f32 / 255.0, self.color.2 as f32 / 255.0, self.color.3 as f32 / 255.0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }


}

impl Component for Canvas {
    fn update(&mut self, app: &mut App) {

        if collides!(app, self, app.mouse.position) {
            // if app.keyboard.held_keys.contains(&"Left Ctrl".to_string()) {
            //     self.rotation += app.mouse.scroll_y as f32 / 100.0;
            // } else
            if app.keyboard.held_keys.contains(&"Left Alt".to_string()) {
                self.scroll_offset = (
                    self.scroll_offset.0 - app.mouse.scroll_x as i64,
                    self.scroll_offset.1 + app.mouse.scroll_y as i64
                )
            } else {
                self.zoom += app.mouse.scroll_y as f32 / 100.0;
                self.zoom = self.zoom.clamp(0.1, 4.0);
            }
        }

        self.render(app);


        // let (_, _, cam_pos) = app.camera.peek();

        app.camera.push();

        // println!("zoom: {} set-pos: {}, {}", self.zoom, dx, dy);
        // app.camera.set_position(dx, -dy);
        
        let dx = ((app.window_size.0 as f32 * self.zoom) - app.window_size.0 as f32) / 2.0;
        let dy = ((app.window_size.1 as f32 * self.zoom) - app.window_size.1 as f32) / 2.0;

        let ox = self.scroll_offset.0 as f32 / 2.0; // (app.window_size.0 as f32 / app.window_size.1 as f32);
        let oy = -(self.scroll_offset.1 as f32 / 2.0);
        
        app.camera.translate(dx + ox, dy + oy, app.window_size);
        app.camera.set_ipos(self.position.0, self.position.1);

        app.camera.set_scale(self.zoom, self.zoom);

        // TODO: add another translation to compensate for rotation
        app.camera.set_rotation(Rad(-self.rotation));
        
        app.camera.viewport = (self.position.0, self.position.1, self.size.0+1, self.size.1+1);

        for child in &mut self.children {
            child.update(app);
        }

        app.camera.pop();

    }

    fn get_named_child(&self, path: VecDeque<&str>) -> Option<&mut dyn Component> {
        None
    }

    fn get_element_name(&self) -> &str {
        &self.uid
    }


    fn destroy(self) {
    }
}

