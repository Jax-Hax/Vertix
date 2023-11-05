use bevy_ecs::system::Resource;

use crate::{assets::AssetServer, camera::CameraStruct, resources::WindowEvents};

#[derive(Resource)]
pub struct App {
    pub asset_server: AssetServer,
    pub camera: CameraStruct,
    pub window_events: WindowEvents,
    pub dt: instant::Duration,
}