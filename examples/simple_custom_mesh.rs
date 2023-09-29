use glam::{Vec3, Quat, Vec2};
use vertix::{prelude::*, camera::{Camera, default_3d_cam}, primitives::rect};
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 0.0, 10.0), f32::to_radians(-90.0), f32::to_radians(0.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
    let instances = vec![Instance::new_with_color(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Quat::IDENTITY,
        [1.0,0.0,0.0,1.0]
    )];
    let (container, is_dynamic) = state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
    match is_dynamic {
        Some(_) => state.world.spawn((container, IsDynamic(), WorldSpace(),)),
        None => state.world.spawn((container, WorldSpace(),)),
    };
    //render loop
    run_event_loop(state, event_loop, None, None, Some(default_3d_cam));
}
