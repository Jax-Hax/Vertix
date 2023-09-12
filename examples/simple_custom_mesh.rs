use vertix::prelude::*;
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR")).await;
    //custom mesh
    let vertices = vec![
        Vertex {
            position: [0.5, 0.5, 0.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];
    let indices = vec![0, 1, 3, 1, 2, 3];
    let instances = vec![Instance {
        position: cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
            .normalize(),
            cgmath::Deg(45.0),
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
        Some(_) => state.world.spawn((container, IsDynamic)),
        None => state.world.spawn((container,)),
    };
    //render loop
    run_event_loop(state, event_loop, update, keyboard_input);
}
fn update(state: &mut State) {
    
}
fn keyboard_input(state: &mut State, event: &KeyboardInput) {
    
}
