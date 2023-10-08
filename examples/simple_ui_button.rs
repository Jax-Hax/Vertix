use glam::{Vec2, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    collision::Box2D,
    prelude::*,
    primitives::rect,
};
use bevy_ecs::prelude::*;
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        f32::to_radians(-90.0),
        f32::to_radians(0.0),
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = vertix::prelude::State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let p1 = Vec2::new(-0.5, -0.5);
    let p2 = Vec2::new(0.5, 0.5);
    let (vertices, indices) = rect(p1,p2);
    let collider = Box2D::new(p1,p2);
    let mut instance = Instance {is_world_space: false, ..Default::default()};
    let mut instances = vec![];
    instances.push(&mut instance);
    state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("rounded_rect.png").await,
        false,
    );
    state.world.spawn((instance, collider));
    state.scheduler.add_systems(movement);
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}
fn movement(mut query: Query<(&mut Instance, &Box2D)>) {
    for (mut instance, collider) in &mut query {
        
    }
}