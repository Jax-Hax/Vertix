use bevy_ecs::system::Resource;
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
}
#[derive(Resource)]
pub struct DeltaTime {
    pub dt: instant::Duration,
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