use glam::{Quat, Vec2, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    collision::Box2D,
    prelude::*,
    primitives::rect,
};
use winit::event::WindowEvent;
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
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let p1 = Vec2::new(-0.5, -0.5);
    let p2 = Vec2::new(0.5, 0.5);
    let (vertices, indices) = rect(p1,p2);
    let collider = Box2D::new(p1,p2);
    let instances = vec![Instance::new_with_color(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Quat::IDENTITY,
        [1.0, 0.0, 0.0, 1.0],
    )];
    let (container, is_dynamic) = state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("rounded_rect.jpg").await,
        false,
    );
    match is_dynamic {
        Some(_) => state.world.spawn((container, collider, IsDynamic)),
        None => state.world.spawn((container, collider)),
    };
    //render loop
    run_event_loop(state, event_loop, None, Some(input), Some(default_3d_cam));
}
fn input(state: &mut State, event: &WindowEvent) {
    //keyboard inputs
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let pos = state.window.normalize_position(position);
            for (_entity, (game_object, collider,)) in state
                .world
                .query_mut::<(&mut InstanceContainer, &Box2D,)>()
            {
                collider.check_collision(&pos);
            }
        }
        _ => {}
    }
}
