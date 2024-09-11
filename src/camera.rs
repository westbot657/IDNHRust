use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};


pub struct Camera2D {
    pub matrix: Matrix4<f32>,
    stack: Vec<Matrix4<f32>>,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::identity(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        // Save the current matrix on the stack
        self.stack.push(self.matrix);
    }

    pub fn pop(&mut self) {
        if let Some(previous_matrix) = self.stack.pop() {
            // Restore the previous matrix
            self.matrix = previous_matrix;
        }
    }

    pub fn apply_transform(&mut self, transform: Matrix4<f32>) {
        self.matrix = self.matrix * transform;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let translation = Matrix4::from_translation(Vector3::new(x, y, 0.0));
        self.apply_transform(translation);
    }

    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        let scale = Matrix4::from_nonuniform_scale(scale_x, scale_y, 1.0);
        self.apply_transform(scale);
    }

    pub fn set_rotation(&mut self, angle: f32) {
        let rotation = Matrix4::from_angle_z(Rad(angle));
        self.apply_transform(rotation);
    }
}
