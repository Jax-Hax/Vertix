use bevy_ecs::system::{ResMut, Resource, Res};
use glam::Vec3;
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*, app_resource::App, collision::structs_3d::{OrientedBoundingBox, Ray}, shapes::cube,
};

fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 5.0, 10.0),
        f32::to_radians(-90.0),
        f32::to_radians(-20.0),
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    let mut instance = Instance {
        ..Default::default()
    };
    let obb = OrientedBoundingBox::new(2.,2.,2.);
    let asset_server = &mut state.world.get_resource_mut::<App>().unwrap().asset_server;
    let mat_idx = asset_server
        .compile_material(
            "cube-diffuse.jpg"
        )
        .await;
    asset_server
        .build_mesh(
            cube(2.,2.,2.),
            vec![&mut instance],
            mat_idx,
            true,
        );
    state.world.insert_resource(Mat {idx: mat_idx});
    state.world.spawn((instance,obb));
    state.schedule.add_systems(movement);
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}
#[derive(Resource)]
struct Mat {
    pub idx: usize
}
fn movement(
    mut app: ResMut<App>,
    mat: Res<Mat>
) {
    if app.window_events.left_clicked() {
        let ray = Ray {origin: app.camera.camera_transform.position, direction: app.window_events.mouse_ray_direction};
        
        let collided = OrientedBoundingBox::new(2.,2.,2.).check_collision_with_ray(Ray {origin: app.camera.camera_transform.position, direction: app.window_events.mouse_ray_direction}, &Instance{..Default::default()});
        app.draw_ray(ray, 100., mat.idx);
        if collided.is_some() {
            
        }
    }
}