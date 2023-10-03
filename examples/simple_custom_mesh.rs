use glam::{Vec3, Quat, Vec2};
use hecs::World;
use vertix::{prelude::*, camera::{Camera, default_3d_cam}, primitives::rect, structs::InstanceRaw};
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
    state.world.spawn((Instance {is_world_space: false, ..Default::default()},));
    let instances = update_instances(&mut state.world);
    let container = state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
    state.world.spawn((container,));
    //render loop
    run_event_loop(state, event_loop, None, None, Some(default_3d_cam));
}
fn update_instances(world: &mut World) -> Vec<InstanceRaw>{
    let mut instances = vec![];
    for (_entity, (game_object,)) in world
        .query_mut::<(&mut Instance,)>()
    {
        instances.push(game_object.to_raw());
    }
    instances
}