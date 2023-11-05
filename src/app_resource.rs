use bevy_ecs::system::Resource;
use winit::dpi::PhysicalPosition;

use crate::{assets::AssetServer, camera::CameraStruct, resources::WindowEvents};

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
}