use glam::{Vec4, Vec3, Mat4, vec3};
use winit::{dpi::PhysicalPosition, event::{VirtualKeyCode, ElementState}};

use crate::camera::{Camera, Projection};
pub struct WindowEvents {
    pub keys_pressed: Vec<(VirtualKeyCode, ElementState)>,
    pub screen_mouse_pos: PhysicalPosition<f32>,
    pub world_mouse_pos: PhysicalPosition<f32>,
    pub left_mouse: MouseClickType,
    pub right_mouse: MouseClickType,
    pub middle_mouse: MouseClickType,
    pub aspect_ratio: f32,
    pub mouse_ray_direction: Vec3,
}
pub enum MouseClickType{
    Clicked,
    Held,
    Released,
    NotHeld
}
impl WindowEvents {
    pub fn is_key_pressed(&self, key: VirtualKeyCode, press_type: Option<ElementState>) -> bool {
        for (key_pressed, pressed_type) in &self.keys_pressed{
            if key_pressed == &key {
                if press_type.is_none() {
                    return true
                }
                else{
                    if pressed_type == &press_type.unwrap() {
                        return true
                    }
                    return false
                }
            }
        }
        false
    }
    pub fn left_clicked(&self) -> bool {
        if let MouseClickType::Clicked = self.left_mouse{
            return true;
        }
        false
    }
    pub fn left_held(&self) -> bool {
        if let MouseClickType::Held = self.left_mouse{
            return true;
        }
        false
    }
    pub fn next_frame(&mut self) {
        self.keys_pressed = vec![];
        match self.left_mouse {
            MouseClickType::Clicked => self.left_mouse = MouseClickType::Held,
            MouseClickType::Released => self.left_mouse = MouseClickType::NotHeld,
            _ => {}
        }
        match self.right_mouse {
            MouseClickType::Clicked => self.left_mouse = MouseClickType::Held,
            MouseClickType::Released => self.left_mouse = MouseClickType::NotHeld,
            _ => {}
        }
        match self.middle_mouse {
            MouseClickType::Clicked => self.left_mouse = MouseClickType::Held,
            MouseClickType::Released => self.left_mouse = MouseClickType::NotHeld,
            _ => {}
        }
    }
    pub fn update_mouse_pos(&mut self, normalized_mouse_pos: PhysicalPosition<f32>, camera_transform: &Camera){
        self.screen_mouse_pos = normalized_mouse_pos;
        self.world_mouse_pos = PhysicalPosition::new(normalized_mouse_pos.x + camera_transform.position.x, normalized_mouse_pos.y + camera_transform.position.y);
    }
    pub fn update_mouse_pos_with_cam_if_cam_2d(&mut self, camera_transform: &mut Camera){ //only works if cam is in 2d and isnt rotated
        let normalized_mouse_pos = self.screen_mouse_pos;
        self.world_mouse_pos = PhysicalPosition::new(normalized_mouse_pos.x + camera_transform.position.x, normalized_mouse_pos.y + camera_transform.position.y);
    }
    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) /(height as f32);
    }
    pub fn calculate_mouse_dir(&mut self, proj_matrix: &Projection, view_matrix: &[[f32; 4]; 4]) {
        let ray_clip_start = Vec4::new(-self.screen_mouse_pos.x, -self.screen_mouse_pos.y, -1.0, 1.0); //screen mosue pos is (-1,-1) to (1,1)
        let ray_clip_end = Vec4::new(-self.screen_mouse_pos.x, -self.screen_mouse_pos.y, 0.0, 1.0); //screen mosue pos is (-1,-1) to (1,1)
        let view_mat = Mat4::from_cols_array_2d(&view_matrix);
        let inversed_view_and_proj_m = (proj_matrix.calc_matrix() * view_mat).inverse();
        let mut ray_wor_start = inversed_view_and_proj_m * ray_clip_start;
        ray_wor_start /= ray_wor_start.w;
        let mut ray_wor_end = inversed_view_and_proj_m * ray_clip_end;
        ray_wor_end /= ray_wor_end.w;
        let ray_dir_world = (ray_wor_end - ray_wor_start).normalize();
        self.mouse_ray_direction = vec3(-ray_dir_world.x, -ray_dir_world.y, -ray_dir_world.z);
    }
}
pub struct Timer {
    pub time_left: time::Duration,
}
impl Timer {
    pub fn finished(&self) -> bool {
        if self.time_left.is_negative() || self.time_left.is_zero() {
            return true
        }
        false
    }
    pub fn tick(&mut self, delta_time: instant::Duration) {
        self.time_left -= delta_time;
    }
    pub fn new(duration: instant::Duration) -> Self {
        Self {time_left: time::Duration::new(duration.as_secs() as i64, duration.subsec_nanos() as i32)}
    }
}