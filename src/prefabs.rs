use wgpu::{Buffer, Queue};
use hecs::World;
use crate::{prelude::InstanceRaw, structs::MeshType};

/// 
/// user will use it like:
/// Prefab {
///     Graphics
/// }
/// 
/// 
pub struct EventHandler{
    pub update_fn: Option<fn(&mut World)>, //called once per frame
    pub collision: Option<fn(&mut World)>, //called when a collision on this is detected
    pub on_click: Option<fn(&mut World)>, //called when it is clicked (if it has a collider)
    pub to_raw: fn(&mut World)-> Vec<InstanceRaw>, //called whenever it needs to be updated
}
pub struct Prefab {
    pub length: u32,
    pub buffer: Buffer,
    pub mesh_type: MeshType,
    pub is_changed: bool, //set each time an instance is changed and it is remade at end of frame
    pub event_handler: EventHandler,
}
impl Prefab {
    pub fn new(buffer: Buffer, mesh_type: MeshType, length: u32,  event_handler: EventHandler) -> Self {
        Self {
            buffer,
            mesh_type,
            length,
            is_changed: false,
            event_handler,
        }
    }
    pub fn update_buffer(&mut self, instances: Vec<InstanceRaw>, queue: &Queue) {
        //optional, must call after you change position or rotation to update it in buffer, also when you add an instance
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&instances));
        self.length = instances.len() as u32;
    }
}