use glam::{Vec3, Quat};
use vertix::{prelude::*, structs::WorldSpace, camera::{Camera, default_3d_cam}};
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 5.0, 10.0), f32::to_radians(-90.0), f32::to_radians(-20.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
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
    run_event_loop(state, event_loop, update, keyboard_input, default_3d_cam);
}
fn update(_state: &mut State) {}
fn keyboard_input(_state: &mut State, _event: &KeyboardInput) {}
