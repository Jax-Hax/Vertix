use glam::{Vec2, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
    collision::structs_2d::Box2D,
    shapes::rect, app_resource::App,
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
    let collider = Box2D::new(p1,p2);
    let mut instance = Instance {is_world_space: false, ..Default::default()};
    let mut instances = vec![];
    instances.push(&mut instance);
    let asset_server = &mut state.world.get_resource_mut::<App>().unwrap().asset_server;
    let material_idx = asset_server.compile_material("rounded_rect.png", wgpu::FilterMode::Linear).await;
    asset_server.build_mesh(
        rect(1.,1.),
        instances,
        material_idx,
        false,
    );
    state.world.spawn((instance, collider));
    state.schedule.add_systems(movement);
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}
fn movement(query: Query<(&Instance, &Box2D)>, app: Res<App>) {
    for (instance, collider) in &query {
        if collider.check_collision(instance, &app.window_events) {
            //println!("collision");
            if app.window_events.left_clicked() {
                println!("click")
            }
        }
    }
}