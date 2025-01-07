use std::collections::VecDeque;
use std::ops::Mul;
use cgmath::{Matrix, Matrix3, Matrix4, Quaternion, Rad, SquareMatrix, Vector3};

type Viewport = (i32, i32, u32, u32);
type Entry = (Matrix4<f64>, Matrix3<f64>, bool, Viewport);

// pub struct Camera {
//     pub matrix: Matrix4<f32>,
//     pub viewport: (i32, i32, u32, u32),
//     pub position: (i32, i32),
//     stack: VecDeque<(Matrix4<f32>, (i32, i32, u32, u32), (i32, i32))>,
// }

pub struct Camera {
    stack: VecDeque<Entry>,
    pub position: Vector3<f64>,
    pub rotation: Quaternion<f64>
}

struct MatrixUtil {}
impl MatrixUtil {
    pub fn is_translation(matrix: Matrix4<f64>) -> bool {

        false
    }

    pub fn is_orthonormal(matrix: Matrix4<f64>) -> bool {

        false
    }

}


impl Camera {
    pub fn new() -> Self {
        let mut stack = VecDeque::new();
        stack.push_back((
            Matrix4::identity(),
            Matrix3::identity(),
            true,
            (i32::MIN, i32::MIN, u32::MAX, u32::MAX)
            ) as Entry
        );
        Self {
            stack,
            position: Vector3::new(0f64, 0f64, 0f64),
            rotation: Quaternion::new(0f64, 0f64, 0f64, 0f64),
        }
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        let &mut entry = self.stack.back_mut().unwrap();
        *entry.3 = viewport;
    }

    pub fn get_viewport(&self) -> Viewport {
        let entry = self.stack.back().unwrap();
        entry.3
    }

    pub fn get_parent_viewport(&mut self) -> Viewport {
        if self.stack.len() >= 2 {
            let back_entry = self.stack.pop_back().unwrap();
            let parent_entry = self.stack.back().unwrap();
            self.stack.push_back(back_entry);
            parent_entry.3
        } else {
            (i32::MIN, i32::MIN, u32::MAX, u32::MAX)
        }
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        let &mut entry = self.stack.back_mut().unwrap();
        *entry.0 += Vector3::new(x, y, z);
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        let &mut entry = self.stack.back_mut().unwrap();
        let scale = Matrix4::from_scale(Vector3::new(x, y, z));
        *entry.0 *= scale;
        if x.abs() == y.abs() && y.abs() == z.abs() {
            if x < 0f64 || y < 0f64 || z < 0f64 {
                let scale = Matrix3::from_scale(Vector3::new(x.signum(), y.signum(), z.signum()));
                *entry.1 *= scale;
            }
        } else {
            let scale = Matrix3::from_scale(Vector3::new(1f64/x, 1f64/y, 1f64/z));
            *entry.1 *= scale;
            *entry.2 = false;
        }
    }

    pub fn multiply(&mut self, quaternion: Quaternion<f64>) {
        let &mut entry = self.stack.back_mut().unwrap();
        *entry.0 *= quaternion;
        *entry.1 *= quaternion;
    }

    pub fn rotate(&mut self, x: Rad<f64>, y: Rad<f64>, z: Rad<f64>) {
        self.multiply(Quaternion::from(Vector3::new(x, y, z)));
    }

    pub fn rotate_around(&mut self, rotation: Quaternion<f64>, origin: Vector3<f64>) {
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
        let &mut entry = self.stack.back_mut().unwrap();
        *entry.0 = Matrix4::identity();
        *entry.1 = Matrix3::identity();
        *entry.2 = true;
    }

    fn compute_entry_normal(entry: &mut Entry) {
        *entry.1 = *entry.0.invert().unwrap().transpose()
    }

    pub fn multiply_position_matrix(&mut self, matrix: Matrix4<f64>) {
        let &mut entry = self.stack.back_mut().unwrap();
        *entry.0 = *entry.0.mul(matrix);

        if !MatrixUtil::is_translation(matrix) {
            if (MatrixUtil::is_orthonormal(matrix)) {
                *entry.1 = *entry.1.mul(matrix);
            } else {
                self.compute_entry_normal(&*entry);
            }
        }

    }


}


// impl Camera {
//     pub fn new(screen_width: u32, screen_height:u32) -> Self {
//
//         Self {
//             matrix: Matrix4::identity(),
//             viewport: (0, 0, screen_width, screen_height),
//             position: (0, 0),
//             stack: VecDeque::new(),
//         }
//     }
//
//     pub fn project(&mut self, screen_width: u32, screen_height: u32) {
//         let aspect_ratio = screen_width as f32 / screen_height as f32;
//
//         // Set up an orthographic projection matrix with aspect ratio correction
//         let left = -1.0 * aspect_ratio;
//         let right = 1.0 * aspect_ratio;
//         let bottom = -1.0;
//         let top = 1.0;
//
//         // Adjust the orthographic projection to account for aspect ratio
//         let projection = ortho(left, right, bottom, top, -1.0, 1.0);
//         if self.stack.is_empty() {
//             self.matrix = projection;
//             self.viewport = (0, 0, screen_width, screen_height);
//         }
//         else {
//             self.stack[0].0 = projection;
//             self.stack[0].1 = (0, 0, screen_width, screen_height);
//         }
//     }
//
//     pub fn push(&mut self) {
//         self.stack.push((self.matrix, self.viewport, self.position));
//         self.matrix = Matrix4::identity();
//         self.viewport = (0, 0, self.viewport.2, self.viewport.3);
//         self.position = (0, 0);
//     }
//
//     pub fn pop(&mut self) {
//         if let Some(previous_matrix) = self.stack.pop() {
//             self.matrix = previous_matrix.0;
//             self.viewport = previous_matrix.1;
//             self.position = previous_matrix.2;
//         }
//     }
//
//     pub fn peek(&self) -> (Matrix4<f32>, (i32, i32, u32, u32), (i32, i32)) {
//         let mut mat_out: Matrix4<f32> = Matrix4::identity();
//
//         let mut dx = 0;
//         let mut dy = 0;
//         let mut x = 0;
//         let mut y = 0;
//         let mut vmx = u32::MAX;
//         let mut vmy = u32::MAX;
//
//         for mat in &self.stack {
//             mat_out = mat_out * mat.0;
//             dx += mat.1.0;
//             dy += mat.1.1;
//             vmx = vmx.min(mat.1.2);
//             vmy = vmy.min(mat.1.3);
//             x += mat.2.0;
//             y += mat.2.1;
//         }
//         mat_out = mat_out * self.matrix;
//         (mat_out, (self.viewport.0 + dx + x, self.viewport.1 + dy + y, self.viewport.2.min(vmx), self.viewport.3.min(vmy)), (x + self.position.0, y + self.position.1))
//     }
//
//     pub fn apply_transform(&mut self, transform: Matrix4<f32>) {
//         self.matrix = self.matrix * transform;
//     }
//
//     pub fn set_ipos(&mut self, x: i32, y: i32) {
//         self.position = (x, y);
//     }
//
//     pub fn set_position(&mut self, x: f32, y: f32) {
//         let translation = Matrix4::from_translation(Vector3::new(x*2.0, -y*2.0, 0.0));
//         self.apply_transform(translation);
//     }
//
//     pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
//         let scale = Matrix4::from_nonuniform_scale(scale_x, scale_y, 1.0);
//         self.apply_transform(scale);
//     }
//
//     pub fn set_rotation(&mut self, angle: Rad<f32>) {
//         let rotation = Matrix4::from_angle_z(angle);
//         self.apply_transform(rotation);
//     }
//
//     /// dx and dy should be in pixel space
//     pub fn translate(&mut self, dx: f32, dy: f32, window_size: (u32, u32)) {
//         let translation = Matrix4::from_translation(Vector3::new(dx / window_size.1 as f32 * 2.0, -dy / window_size.1 as f32 * 2.0, 0.0));
//         self.apply_transform(translation);
//     }
//
// }
