use renderer::State;
use util::CameraUniform;
use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
mod camera;
mod model;
mod renderer;
mod texture;
mod util;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    util::init();
    let (event_loop, window) = util::make_window();
    let mut state = State::new(window).await;
    let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let mut projection =
        camera::Projection::new(state.config.width, state.config.height, cgmath::Deg(45.0), 0.1, 100.0);
    let camera_controller = camera::CameraController::new(4.0, 0.4);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, &projection);

    let camera_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
    state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let camera_bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });
    let mut last_render_time = instant::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => state.window.request_redraw(),
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    (state, projection) = util::resize(state, *physical_size,projection);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    (state, projection) = util::resize(state, **new_inner_size, projection);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;

                camera_controller.update_camera(&mut camera, dt);
                camera_uniform.update_view_proj(&camera, &projection);
                state.queue.write_buffer(
                    &camera_buffer,
                    0,
                    bytemuck::cast_slice(&[camera_uniform]),
                );
                /*match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        (state, projection) = util::resize(state, state.size,projection)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }*/
            }
            _ => {}
        }
    });
}
