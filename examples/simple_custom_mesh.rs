use glam::Vec3;
use vertix::{prelude::*, camera::{Camera, default_3d_cam}, shapes::rect, assets::AssetServer};
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 0.0, 10.0), f32::to_radians(-90.0), f32::to_radians(0.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let mut instance = Instance {is_world_space: true, ..Default::default()};
    let mut instances = vec![];
    let mut asset_server = state.world.get_resource_mut::<AssetServer>().unwrap();
    instances.push(&mut instance);
    let material_idx = asset_server.compile_material("cube-diffuse.jpg").await;
    asset_server.build_mesh(
        rect(1.,1.),
        instances,
        material_idx,
        false,
    );
    state.world.spawn((instance,));
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}