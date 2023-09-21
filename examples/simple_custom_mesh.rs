use glam::{Vec3, Quat};
use instant::Duration;
use vertix::{prelude::*, engine::WorldSpace, camera::Camera};
use std::f32::consts::FRAC_PI_2;
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 5.0, 10.0), f32::to_radians(-90.0), f32::to_radians(-20.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 4.0, 0.4).await;
    //custom mesh
    let vertices = vec![
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            tex_coords: [0.4, 0.4],
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            tex_coords: [0.5, 0.5],
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            tex_coords: [0.6, 0.6],
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            tex_coords: [0.7, 0.7],
        }, // D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            tex_coords: [0.8, 0.8],
        }, // E
    ];

    let indices = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
    let instances = vec![Instance {
        position: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        rotation: Quat::from_axis_angle(
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            .normalize(),
            f32::to_radians(45.0),
        ),
    }];
    let (container, is_dynamic) = state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
    match is_dynamic {
        Some(_) => state.world.spawn((container, IsDynamic,WorldSpace)),
        None => state.world.spawn((container,WorldSpace)),
    };
    //render loop
    run_event_loop(state, event_loop, update, keyboard_input, update_camera);
}
fn update_camera(state: &mut State, dt: Duration) {
    let dt = dt.as_secs_f32();
    let mut camera = &mut state.camera.camera_transform;
    let mut controller = &mut state.camera.camera_controller;
    // Move forward/backward and left/right
    let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
    let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
    camera.position += forward * (controller.amount_forward - controller.amount_backward) * controller.speed * dt;
    camera.position += right * (controller.amount_right - controller.amount_left) * controller.speed * dt;

    // Move in/out (aka. "zoom")
    // Note: this isn't an actual zoom. The camera's position
    // changes when zooming. I've added this to make it easier
    // to get closer to an object you want to focus on.
    let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
    let scrollward =
    Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
    camera.position += scrollward * controller.scroll * controller.speed * controller.sensitivity * dt;
    controller.scroll = 0.0;

    // Move up/down. Since we don't use roll, we can just
    // modify the y coordinate directly.
    camera.position.y += (controller.amount_up - controller.amount_down) * controller.speed * dt;

    // Rotate
    camera.yaw += controller.rotate_horizontal * controller.sensitivity * dt;
    camera.pitch += -controller.rotate_vertical * controller.sensitivity * dt;

    // If process_mouse isn't called every frame, these values
    // will not get set to zero, and the camera will rotate
    // when moving in a non cardinal direction.
    controller.rotate_horizontal = 0.0;
    controller.rotate_vertical = 0.0;

    // Keep the camera's angle from going too high/low.
    if camera.pitch < -SAFE_FRAC_PI_2 {
        camera.pitch = -SAFE_FRAC_PI_2;
    } else if camera.pitch > SAFE_FRAC_PI_2 {
        camera.pitch = SAFE_FRAC_PI_2;
    }
}
fn update(_state: &mut State) {}
fn keyboard_input(_state: &mut State, _event: &KeyboardInput) {}
