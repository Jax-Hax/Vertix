use glam::{Vec4, Mat4, Vec3};
use winit::dpi::PhysicalPosition;

use crate::camera::Projection;

pub fn calculate_mouse_dir(mouse_coords: PhysicalPosition<f32> /*(-1,-1) to (1,1)*/, proj_matrix: Projection, view_matrix: [[f32; 4]; 4]) {
    let ray_clip = Vec4::new(mouse_coords.x, mouse_coords.y, -1.0, 1.0);
    let mut ray_eye = proj_matrix.calc_matrix().inverse() * ray_clip;
    ray_eye = Vec4::new(ray_eye.x, ray_eye.y, -1.0, 0.0);
    let view_mat = Mat4::from_cols_array_2d(&view_matrix);
    let ray_wor = view_mat.inverse() * ray_eye;
    let ray_world = Vec3::new(ray_wor.x,ray_wor.y,ray_wor.z).normalize();
    println!("{}", ray_world);
}