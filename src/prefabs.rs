use bevy_ecs::world::World;

use crate::prelude::InstanceRaw;

/// 
/// user will use it like:
/// Prefab {
///     Graphics
/// }
/// 
/// 
pub struct Graphics {
    pub length: u32,
    pub buffer: Buffer,
    pub mesh_type: MeshType,
}
impl Graphics {
    pub fn new(buffer: Buffer, mesh_type: MeshType, length: u32) -> Self {
        Self {
            buffer,
            mesh_type,
            length,
        }
    }
    pub fn update(&mut self, instances: Vec<InstanceRaw>, queue: &Queue) {
        //optional, must call after you change position or rotation to update it in buffer, also when you add an instance
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&instances));
        self.length = instances.len() as u32;
    }
}

pub trait OnClick{
    fn on_click(&self, world: World);
}
pub trait Update{
    fn update(&self, world: World);
}
pub trait Collision{
    fn collision(&self, world: World);
}