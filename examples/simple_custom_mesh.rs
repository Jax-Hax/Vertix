use glam::{Vec3, Quat};
use vertix::{prelude::*, structs::WorldSpace, camera::{Camera, default_3d_cam}};
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 5.0, 10.0), f32::to_radians(0.0), f32::to_radians(0.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let vertices = vec![
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [0.4, 0.4],
        }, // A
        Vertex {
            position: [-0.5, 1., 0.0],
            tex_coords: [0.5, 0.5],
        }, // B
        Vertex {
            position: [1., 0., 0.0],
            tex_coords: [0.6, 0.6],
        }, // C
        Vertex {
            position: [1., 1., 0.0],
            tex_coords: [0.7, 0.7],
        }, // D
    ];

    let indices = vec![2,1,0, 1, 2, 3];
    let instances = vec![Instance {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quat::IDENTITY,
    }];
    let (container, is_dynamic) = state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
    match is_dynamic {
        Some(_) => state.world.spawn((container, IsDynamic)),
        None => state.world.spawn((container,)),
    };
    //render loop
    run_event_loop(state, event_loop, update, keyboard_input, default_3d_cam);
}
fn update(_state: &mut State) {}
fn keyboard_input(_state: &mut State, _event: &KeyboardInput) {}
