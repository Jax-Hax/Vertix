use std::iter;
use crate::{state::State, structs::MeshType, model::DrawModel, app_resource::App};

pub fn render(state: &mut State) -> Result<(), wgpu::SurfaceError> {
    let output = state.window.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let app = state.world
    .get_resource::<App>()
    .unwrap();
    let mut encoder = app.asset_server
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
        render_pass.set_bind_group(1, &app.camera.bind_group, &[]);
        for (_, game_object) in &app.asset_server.prefab_slab {
            render_pass.set_vertex_buffer(1, game_object.buffer.slice(..));
            match &game_object.mesh_type {
                MeshType::Model(model) => {
                    render_pass.draw_model_instanced(&model, 0..game_object.length);
                }
                MeshType::Mesh(mesh) => {
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    let material = &app.asset_server.material_assets[mesh.material_idx];
                    render_pass.set_bind_group(0, &material.bind_group, &[]);
                    render_pass.draw_indexed(0..mesh.num_elements, 0, 0..game_object.length);
                }
                MeshType::Sprite(material_idx) => {
                    render_pass.set_vertex_buffer(0, app.asset_server.sprite_mesh.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(app.asset_server.sprite_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    let material = &app.asset_server.material_assets[*material_idx];
                    render_pass.set_bind_group(0, &material.bind_group, &[]);
                    render_pass.draw_indexed(0..app.asset_server.sprite_mesh.num_elements, 0, 0..game_object.length);
                }
            }
        }
    }

    app.asset_server.queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
}
