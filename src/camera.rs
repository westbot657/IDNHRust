use cgmath::{ortho, Matrix4, Rad, SquareMatrix, Vector3};


pub struct Camera {
    pub matrix: Matrix4<f32>,
    pub viewport: (i32, i32, u32, u32),
    stack: Vec<(Matrix4<f32>, (i32, i32, u32, u32))>,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height:u32) -> Self {

        Self {
            matrix: Matrix4::identity(),
            viewport: (0, 0, screen_width, screen_height),
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
        self.stack.push((self.matrix, self.viewport));
        self.matrix = Matrix4::identity();
        self.viewport = (self.viewport.0, self.viewport.1, self.viewport.2, self.viewport.3)
    }

    pub fn pop(&mut self) {
        if let Some(previous_matrix) = self.stack.pop() {
            self.matrix = previous_matrix.0;
            self.viewport = previous_matrix.1;
        }
    }

    pub fn peek(&self) -> (Matrix4<f32>, (i32, i32, u32, u32)) {
        let mut mat_out: Matrix4<f32> = Matrix4::identity();

        for mat in &self.stack {
            mat_out = mat_out * mat.0;
        }
        mat_out = mat_out * self.matrix;
        (mat_out, self.viewport)
    }

    pub fn apply_transform(&mut self, transform: Matrix4<f32>) {
        self.matrix = self.matrix * transform;
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
}
