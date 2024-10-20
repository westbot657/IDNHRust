use cgmath::{ortho, Matrix4, Rad, SquareMatrix, Vector3, Vector4};



pub struct Camera {
    pub matrix: Matrix4<f32>,
    pub viewport: (i32, i32, u32, u32),
    pub position: (i32, i32),
    stack: Vec<(Matrix4<f32>, (i32, i32, u32, u32), (i32, i32))>,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height:u32) -> Self {

        Self {
            matrix: Matrix4::identity(),
            viewport: (0, 0, screen_width, screen_height),
            position: (0, 0),
            stack: Vec::new(),
        }
    }

    pub fn project(&mut self, screen_width: u32, screen_height: u32) {
        let aspect_ratio = screen_width as f32 / screen_height as f32;

        // Set up an orthographic projection matrix with aspect ratio correction
        let left = -1.0 * aspect_ratio;
        let right = 1.0 * aspect_ratio;
        let bottom = -1.0;
        let top = 1.0;

        // Adjust the orthographic projection to account for aspect ratio
        let projection = ortho(left, right, bottom, top, -1.0, 1.0);
        if self.stack.is_empty() {
            self.matrix = projection;
            self.viewport = (0, 0, screen_width, screen_height);
        }
        else {
            self.stack[0].0 = projection;
            self.stack[0].1 = (0, 0, screen_width, screen_height);
        }
    }

    pub fn push(&mut self) {
        self.stack.push((self.matrix, self.viewport, self.position));
        self.matrix = Matrix4::identity();
        self.viewport = (0, 0, self.viewport.2, self.viewport.3);
        self.position = (0, 0);
    }

    pub fn pop(&mut self) {
        if let Some(previous_matrix) = self.stack.pop() {
            self.matrix = previous_matrix.0;
            self.viewport = previous_matrix.1;
            self.position = previous_matrix.2;
        }
    }

    pub fn peek(&self) -> (Matrix4<f32>, (i32, i32, u32, u32), (i32, i32)) {
        let mut mat_out: Matrix4<f32> = Matrix4::identity();

        let mut dx = 0;
        let mut dy = 0;
        let mut x = 0;
        let mut y = 0;
        let mut vmx = u32::MAX;
        let mut vmy = u32::MAX;

        for mat in &self.stack {
            mat_out = mat_out * mat.0;
            dx += mat.1.0;
            dy += mat.1.1;
            vmx = vmx.min(mat.1.2);
            vmy = vmy.min(mat.1.3);
            x += mat.2.0;
            y += mat.2.1;
        }
        mat_out = mat_out * self.matrix;
        (mat_out, (self.viewport.0 + dx + x, self.viewport.1 + dy + y, self.viewport.2.min(vmx), self.viewport.3.min(vmy)), (x + self.position.0, y + self.position.1))
    }

    pub fn apply_transform(&mut self, transform: Matrix4<f32>) {
        self.matrix = self.matrix * transform;
    }

    pub fn set_ipos(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let translation = Matrix4::from_translation(Vector3::new(x*2.0, -y*2.0, 0.0));
        self.apply_transform(translation);
    }

    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        let scale = Matrix4::from_nonuniform_scale(scale_x, scale_y, 1.0);
        self.apply_transform(scale);
    }

    pub fn set_rotation(&mut self, angle: Rad<f32>) {
        let rotation = Matrix4::from_angle_z(angle);
        self.apply_transform(rotation);
    }

    /// dx and dy should be in pixel space
    pub fn translate(&mut self, dx: f32, dy: f32, window_size: (u32, u32)) {
        let translation = Matrix4::from_translation(Vector3::new(dx / window_size.1 as f32 * 2.0, -dy / window_size.1 as f32 * 2.0, 0.0));
        self.apply_transform(translation);
    }
    
    /// Maps a camera-relative rectangle to a window-space rectangle
    /// Useful for mapping the camera's viewport to an object's bounding box regardless of transformations (except rotations)
    pub fn map_rect(&mut self, rect_in: (i32, i32, u32, u32), screen_size: (u32, u32)) -> (i32, i32, u32, u32) {
        let (x, y, width, height) = rect_in;
        let (screen_width, screen_height) = screen_size;

        let aspect_ratio = screen_width as f32 / screen_height as f32;

        let ndc_x = |pixel_x: f32| -> f32 { (pixel_x / screen_height as f32) * 2.0 - 1.0 };
        let ndc_y = |pixel_y: f32| -> f32 { 1.0 - (pixel_y / screen_height as f32) * 2.0 };

        let top_left = Vector4::new(ndc_x(x as f32 * aspect_ratio), ndc_y(y as f32), 0.0, 1.0);
        let top_right = Vector4::new(ndc_x((x + width as i32) as f32 * aspect_ratio), ndc_y(y as f32), 0.0, 1.0);
        let bottom_left = Vector4::new(ndc_x(x as f32 * aspect_ratio), ndc_y((y + height as i32) as f32), 0.0, 1.0);
        let bottom_right = Vector4::new(ndc_x((x + width as i32) as f32 * aspect_ratio), ndc_y((y + height as i32) as f32), 0.0, 1.0);

        let matrix = self.peek().0;

        let transformed_top_left = matrix * top_left;
        let transformed_top_right = matrix * top_right;
        let transformed_bottom_left = matrix * bottom_left;
        let transformed_bottom_right = matrix * bottom_right;

        let xs = [
            transformed_top_left.x / transformed_top_left.w,
            transformed_top_right.x / transformed_top_right.w,
            transformed_bottom_left.x / transformed_bottom_left.w,
            transformed_bottom_right.x / transformed_bottom_right.w,
        ];

        let ys = [
            transformed_top_left.y / transformed_top_left.w,
            transformed_top_right.y / transformed_top_right.w,
            transformed_bottom_left.y / transformed_bottom_left.w,
            transformed_bottom_right.y / transformed_bottom_right.w,
        ];

        let pixel_x = |ndc_x: f32| -> i32 { ((ndc_x + 1.0) / 2.0 * screen_height as f32) as i32 };
        let pixel_y = |ndc_y: f32| -> i32 { (1.0 - (ndc_y) / 2.0 * screen_height as f32) as i32 };

        let min_x = xs.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_x = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let min_y = ys.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_y = ys.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let new_width = (pixel_x(max_x) - pixel_x(min_x)) as u32;
        let new_height = (pixel_y(min_y) - pixel_y(max_y)) as u32;

        (
            pixel_x(min_x),
            pixel_y(min_y),
            new_width,
            new_height,
        )
    }



}
