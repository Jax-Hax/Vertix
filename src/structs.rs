use crate::model::{Material, Model};
use glam::{Mat4, Quat, Vec3};
use bevy_ecs::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, VirtualKeyCode},
};
pub enum MeshType {
    Model(Model),
    SingleMesh(SingleMesh),
}
pub struct SingleMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: Material,
}
#[derive(Debug, Copy, Clone, Component)]
pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
    pub color: [f32; 4],
    pub is_world_space: bool,
    pub container_index: usize,
}
impl Default for Instance {
    fn default() -> Self {
        Instance { position: Vec3::ZERO, rotation: Quat::IDENTITY, color: [1.0,1.0,1.0,1.0], is_world_space: true, container_index: 0 }
    }
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw::new(self.position, self.rotation, self.color, self.is_world_space)
    }
    /*pub fn update(&self, instances: Vec<InstanceRaw>, state: &mut State) {
        state.entity_containers[self.container_index].update(instances, &state.queue);
    }*/
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct CameraController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    pub amount_up: f32,
    pub amount_down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 3.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }
}
