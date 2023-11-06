use bevy_ecs::system::Resource;
use glam::Vec3;
use winit::dpi::PhysicalPosition;

use crate::{assets::AssetServer, camera::CameraStruct, resources::WindowEvents, shapes::line_3d, collision::structs_3d::Ray, prelude::Instance};

#[derive(Resource)]
pub struct App {
    pub asset_server: AssetServer,
    pub camera: CameraStruct,
    pub window_events: WindowEvents,
    pub dt: instant::Duration,
}
impl App {
    pub fn cursor_move(&mut self, normalized_position: PhysicalPosition<f32>) {
        self.window_events.update_mouse_pos(normalized_position, &mut self.camera.camera_transform);
        self.window_events.calculate_mouse_dir(&self.camera.projection, &self.camera.camera_uniform.view_proj);
    }
    pub fn draw_ray(&mut self, normalized_ray: Ray, length: f32, material_idx: usize) {
        let line_segment_start = normalized_ray.origin;
        let line_segment_end = normalized_ray.origin + normalized_ray.direction * length;
        self.draw_line_segment(line_segment_start, line_segment_end, material_idx);
    }
    pub fn draw_line_segment(&mut self, line_start: Vec3, line_end: Vec3, material_idx: usize) {
        self.asset_server.build_mesh(line_3d(line_start, line_end), vec![&mut Instance {..Default::default()}], material_idx, false)
    }
}