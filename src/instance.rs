use bevy_ecs::component::Component;
use glam::{Quat, Vec3, Mat4, Vec2};

use crate::resources::UpdateInstance;

#[derive(Debug, Copy, Clone, Component)]
pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
    pub color: [f32; 4],
    pub is_world_space: bool,
    pub prefab_index: usize,
    pub enabled: bool
}
impl Default for Instance {
    fn default() -> Self {
        Instance { position: Vec3::ZERO, rotation: Quat::IDENTITY, color: [1.0,1.0,1.0,1.0], is_world_space: true, prefab_index: 0, enabled: true }
    }
}

impl Instance {
    pub fn to_raw(&self) -> Option<InstanceRaw> {
        if self.enabled {Some(InstanceRaw::new(self.position, self.rotation, self.color, self.is_world_space))} else {None}
    }
    pub fn update(&self, instances: Vec<InstanceRaw>, instance_update: &mut UpdateInstance) {
        instance_update.prefab_slab.get_mut(self.prefab_index).unwrap().update_buffer(instances, &instance_update.queue);
    }
    pub fn pos_2d(&self) -> Vec2 {
        Vec2::new(self.position.x, self.position.y)
    }
    pub fn pos(&self) -> Vec3 {
        self.position
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    color: [f32; 4],
    is_world_space: u32,
}

impl InstanceRaw {
    pub fn new(position: Vec3, rotation: Quat, color: [f32; 4], is_world_space: bool) -> Self {
        Self {
            model: Mat4::from_rotation_translation(rotation, position).to_cols_array_2d(),
            color: color,
            is_world_space: if is_world_space { 1 } else { 0 },
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}