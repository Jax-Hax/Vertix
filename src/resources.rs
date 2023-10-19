use bevy_ecs::system::Resource;
use instant::Duration;
use slab::Slab;
use winit::{dpi::PhysicalPosition, event::{VirtualKeyCode, ElementState}};

use crate::prefabs::Prefab;

#[derive(Resource)]
pub struct MousePos {
    pub pos: PhysicalPosition<f32>,
}
#[derive(Resource)]
pub struct UpdateInstance {
    pub queue: wgpu::Queue,
    pub prefab_slab: Slab<Prefab>,
}
#[derive(Resource)]
pub struct WindowEvents {
    pub keys_pressed: Vec<(VirtualKeyCode, ElementState)>,
}
#[derive(Resource)]
pub struct DeltaTime {
    pub dt: Duration,
}