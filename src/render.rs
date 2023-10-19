use std::iter;

use crate::{
    model::DrawModel,
    state::State,
    structs::MeshType, resources::UpdateInstance,
};

pub fn render(state: &mut State) -> Result<(), wgpu::SurfaceError> {
    let output = state.window.surface.get_current_texture()?;
    let instance_updater = state.world.get_resource::<UpdateInstance>().unwrap();
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &state.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        render_pass.set_pipeline(&state.render_pipeline);
        render_pass.set_bind_group(1, &state.camera.bind_group, &[]);
        for (_, game_object) in &instance_updater.prefab_slab {
            render_pass.set_vertex_buffer(1, game_object.buffer.slice(..));
            match &game_object.mesh_type {
                MeshType::Model(model) => {
                    render_pass.draw_model_instanced(&model, 0..game_object.length);
                }
                MeshType::SingleMesh(mesh) => {
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_bind_group(0, &mesh.material.bind_group, &[]);
                    render_pass.draw_indexed(0..mesh.num_elements, 0, 0..1);
                }
            }
        }
    }

    instance_updater.queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
}