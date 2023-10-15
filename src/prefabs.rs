use bevy_ecs::component::Component;
use wgpu::{Buffer, Queue};
use crate::{prelude::InstanceRaw, structs::MeshType};
#[derive(Component)]
pub struct Prefab {
    pub length: u32,
    pub buffer: Buffer,
    pub mesh_type: MeshType,
    pub is_changed: bool, //set each time an instance is changed and it is remade at end of frame
}
impl Prefab {
    pub fn new(buffer: Buffer, mesh_type: MeshType, length: u32) -> Self {
        Self {
            buffer,
            mesh_type,
            length,
            is_changed: false,
        }
    }
    pub fn update_buffer(&mut self, instances: Vec<InstanceRaw>, queue: &Queue) {
        //optional, must call after you change position or rotation to update it in buffer, also when you add an instance
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&instances));
        self.length = instances.len() as u32;
    }
}