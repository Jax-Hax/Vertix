use bevy_ecs::system::Resource;
use glam::Vec2;
use slab::Slab;
use wgpu::{Device, Queue, util::DeviceExt, BindGroupLayout};

use crate::{material::Material, prelude::{Vertex, Instance}, primitives::rect, prefabs::Prefab, structs::Mesh, loader::load_texture};

#[derive(Resource)]
pub struct AssetServer {
    pub material_assets: Vec<Material>,
    pub device: Device,
    pub queue: Queue,
    pub prefab_slab: Slab<Prefab>,
    pub build_path: String,
    pub texture_bind_group_layout: BindGroupLayout
}
impl AssetServer {
    pub fn new(device: Device, queue: Queue, build_path: String, texture_bind_group_layout: BindGroupLayout) -> Self {
        Self {
            material_assets: vec![],
            device,
            queue,
            prefab_slab: Slab::new(),
            build_path,
            texture_bind_group_layout
        }
    }
    pub fn remove_prefab(&mut self, prefab_idx: usize) {
        self.prefab_slab.remove(prefab_idx);
    }
    pub fn clear_all_prefabs(&mut self) {
        self.prefab_slab.clear();
    }
    pub async fn compile_materials(&mut self, material_paths: Vec<&str>) -> Vec<usize> {
        let mut material_idxs = vec![];
        for material_path in material_paths {
            self.material_assets.push(self.compile_material_internal(&material_path).await);
            material_idxs.push(self.material_assets.len() - 1);
        }
        material_idxs
    }
    pub async fn compile_material(&mut self, material_path: &str) -> usize {
        self.material_assets.push(self.compile_material_internal(&material_path).await);
        self.material_assets.len() - 1
    }
    async fn compile_material_internal(&self, texture_name: &str) -> Material {
        let diffuse_texture =
            load_texture(texture_name, &self.build_path, &self.device, &self.queue)
                .await
                .unwrap();
        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });
        Material {
            bind_group: texture_bind_group,
        }
    }
    pub fn build_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        instances: Vec<&mut Instance>,
        material_idx: usize,
        is_updating: bool
    ) {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let mesh = Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material_idx,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let entry = self.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
    pub fn make_sprites(
        &mut self,
        instances: Vec<&mut Instance>,
        material_idx: usize,
        is_updating: bool
    ) {
        //make sprite mesh
        let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
        let vertex_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            
        let mesh = Mesh {
            vertex_buffer,index_buffer, num_elements: indices.len() as u32,
            material_idx,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let entry = self.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
}