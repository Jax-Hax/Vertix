use instant::Duration;
use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, DeviceEvent, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::{state::State, render::render, resources::{WindowEvents, MousePos, DeltaTime}};

pub fn run_event_loop(
    mut state: State,
    event_loop: EventLoop<()>,
    cam_update: Option<fn (&mut State, dt: std::time::Duration)>,
) {
    let mut last_render_time = instant::Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => state.window().request_redraw(),
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => if state.mouse_pressed || state.mouse_locked {
                state.camera.camera_controller.process_mouse(delta.0, delta.1)
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                state.input(event);
                match event {
                    #[cfg(not(target_arch="wasm32"))]
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
                    WindowEvent::CursorMoved { position, .. } => {
                        let mut mouse_pos = state.world.get_resource_mut::<MousePos>().unwrap();
                        mouse_pos.pos = state.window.normalize_position(position);
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                let mut delta_time = state.world.get_resource_mut::<DeltaTime>().unwrap();
                delta_time.dt = dt;
                if cam_update.is_some() {
                    cam_update.unwrap()(&mut state, dt);
                }
                state.update();
                state.schedule.run(&mut state.world);
                state.world.get_resource_mut::<WindowEvents>().unwrap().keys_pressed = vec![];
                match render(&mut state) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.window.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            _ => {}
        }
    });
}
pub fn delta_time_to_seconds(dt: Duration) -> f32 {
    dt.as_millis() as f32 * 0.001
}