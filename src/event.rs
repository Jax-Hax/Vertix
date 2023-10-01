use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, DeviceEvent, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::{state::State, render::render};

pub fn run_event_loop<T>(
    mut state: State,
    main_state: T,
    event_loop: EventLoop<()>,
    update: Option<fn(&mut State, T)>,
    input: Option<fn(&mut State, &WindowEvent, T)>,
    cam_update: Option<fn (&mut State, dt: std::time::Duration, T)>,
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
                if input.is_some() {
                    input.unwrap()(&mut state,event, main_state);
                }
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
                        state.mouse_pos = position.clone();
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
                if cam_update.is_some() {
                    cam_update.unwrap()(&mut state, dt, main_state);
                }
                state.update();
                if update.is_some() {
                    update.unwrap()(&mut state, main_state);
                }

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
