use wgpu::{Buffer, Queue};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use glam::{Vec3, Quat, Mat4};
use crate::model::{Model, Material};
pub struct IsDynamic;
pub struct ScreenSpace();
pub struct WorldSpace();
pub struct InstanceContainer {
    pub length: u32,
    pub buffer: Buffer,
    pub mesh_type: MeshType,
    pub instances: Vec<Instance>,
}
pub enum MeshType {
    Model(Model),
    SingleMesh(SingleMesh),
}
pub struct SingleMesh{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: Material,
}
impl InstanceContainer {
    pub fn new(buffer: Buffer, mesh_type: MeshType, instances: Vec<Instance>) -> Self {
        Self {
            buffer,
            mesh_type,
            length: instances.len() as u32,
            instances,
        }
    }
    pub fn update(&self, queue: &Queue) {
        //optional, must call after you change position or rotation to update it in buffer, also when you add an instance
        let instance_data = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&instance_data));
    }
}
pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: Mat4::from_rotation_translation(self.rotation,self.position)
            .to_cols_array_2d(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    #[allow(dead_code)]
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
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
            ],
        }
    }
}
