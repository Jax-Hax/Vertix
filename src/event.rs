use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, DeviceEvent, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}, dpi::PhysicalPosition};

use crate::state::State;

#[derive(Default)]
pub struct EventHandler{
    pub update: Option<fn(&mut State)>,
    pub keyboard_input: Option<fn(&mut State, &winit::event::KeyboardInput)>,
    pub mouse_input: Option<fn(&mut State, &winit::event::KeyboardInput)>,
    pub cam_update: Option<fn (&mut State, dt: std::time::Duration)>,
    pub mouse_move: Option<fn(&mut State, &PhysicalPosition<f64>)>,
}

pub fn run_event_loop(
    mut state: State,
    event_loop: EventLoop<()>,
    event_handler: EventHandler
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
                    WindowEvent::KeyboardInput { input, .. } => {
                        if event_handler.keyboard_input.is_some() {
                            event_handler.keyboard_input.unwrap()(&mut state, input);
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if event_handler.mouse_move.is_some() {
                            event_handler.mouse_move.unwrap()(&mut state, position);
                        }
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
                if event_handler.cam_update.is_some() {
                    event_handler.cam_update.unwrap()(&mut state, dt);
                }
                state.update();
                if event_handler.update.is_some() {
                    event_handler.update.unwrap()(&mut state);
                }

                match state.render() {
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
