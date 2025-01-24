use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use cgmath::{Matrix, Matrix3, Matrix4, Quaternion, Rad, SquareMatrix, Vector3, Vector4};

type Viewport = (i32, i32, u32, u32);
type Entry = (Matrix4<f32>, Matrix3<f32>, bool, Viewport);

// pub struct Camera {
//     pub matrix: Matrix4<f32>,
//     pub viewport: (i32, i32, u32, u32),
//     pub position: (i32, i32),
//     stack: VecDeque<(Matrix4<f32>, (i32, i32, u32, u32), (i32, i32))>,
// }

pub struct Camera {
    stack: VecDeque<Entry>,
    pub aspect_ratio: f32,
    pub window_height: u32,
}

impl Camera {
    pub fn new() -> Self {
        let mut stack = VecDeque::new();
        stack.push_back((
            Matrix4::identity(),
            Matrix3::identity(),
            true,
            (0, 0, 10000, 10000)
            )
        );
        Self {
            stack,
            aspect_ratio: 0.0,
            window_height: 0
        }
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        let pvp = self.get_parent_viewport();
        let entry = self.stack.back_mut().unwrap();
        entry.3 = (
            pvp.0 + viewport.0,
            pvp.1 + viewport.1,
            viewport.2,
            viewport.3,
        );
    }

    pub fn get_viewport(&self) -> Viewport {
        let entry = self.stack.back().unwrap();
        entry.3
    }

    pub fn get_parent_viewport(&mut self) -> Viewport {
        if self.stack.len() >= 2 {
            let back_entry = self.stack.pop_back().unwrap();
            let parent_entry = Entry::from(*self.stack.back().unwrap());
            self.stack.push_back(back_entry);
            parent_entry.3
        } else {
            (0, 0, 10000, 10000)
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let entry = self.stack.back_mut().unwrap();
        entry.0 = entry.0 * Matrix4::from_translation(Vector3::new(x, y, z));
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let entry = self.stack.back_mut().unwrap();
        let scale = Matrix4::from_nonuniform_scale(x, y, z);
        entry.0 = entry.0 * scale;
        if x.abs() == y.abs() && y.abs() == z.abs() {
            if x < 0f32 || y < 0f32 || z < 0f32 {
                let scale = Matrix3::from_nonuniform_scale(x, y);
                entry.1 = entry.1 * scale;
            }
        } else {
            let scale = Matrix3::from_nonuniform_scale(1f32/x, 1f32/y);
            entry.1 = entry.1 * scale;
            entry.2 = false;
        }
    }

    pub fn multiply(&mut self, quaternion: Quaternion<f32>) {
        let entry = self.stack.back_mut().unwrap();
        entry.0 = entry.0 * Matrix4::from(quaternion);

        entry.1 = entry.1 * Matrix3::from(quaternion);
    }

    pub fn rotate(&mut self, x: Rad<f32>, y: Rad<f32>, z: Rad<f32>) {
        self.multiply(Quaternion::from_sv(1f32, Vector3::new(x.0, y.0, z.0)));
    }

    pub fn rotate_around(&mut self, rotation: Quaternion<f32>, origin: Vector3<f32>) {
        let entry = self.stack.back_mut().unwrap();

        let translation_matrix = &mut entry.0;
        let normal_matrix = &mut entry.1;

        let to_origin = Matrix4::from_translation(-origin);
        let back_to_position = Matrix4::from_translation(origin);
        let rotation_matrix = Matrix4::from(rotation);

        *translation_matrix = back_to_position * rotation_matrix * to_origin * *translation_matrix;

        let normal_rotation = Matrix3::from(rotation);
        *normal_matrix = normal_rotation * *normal_matrix;
    }


    pub fn push(&mut self) {
        self.stack.push_back(
            self.stack.back().unwrap().clone()
        )
    }

    pub fn pop(&mut self) {
        self.stack.pop_back();
    }

    pub fn peek(&self) -> &Entry {
        self.stack.back().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }

    pub fn load_identity(&mut self) {
        let entry = self.stack.back_mut().unwrap();
        entry.0 = Matrix4::identity();
        entry.1 = Matrix3::identity();
        entry.2 = true;
    }

    pub fn matrix3(matrix: &Matrix4<f32>) -> Matrix3<f32> {
        Matrix3::new(
            matrix[0][0], matrix[0][1], matrix[0][2],
            matrix[1][0], matrix[1][1], matrix[1][2],
            matrix[2][0], matrix[2][1], matrix[2][2],
        )
    }

    fn compute_entry_normal(entry: &mut Entry) {
        (*entry).1 = Self::matrix3(&((*entry).0.invert().unwrap().transpose()));
    }

}
