use bevy_ecs::system::{Query, Res, ResMut};
use glam::Vec3;
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*, collision::OrientedBoundingBox, app_resource::App,
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
    asset_server
        .create_model_instances(
            "cube.obj",
            vec![&mut instance],
            true,
        )
        .await;
    state.world.spawn((instance,obb));
    state.schedule.add_systems(movement);
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}
fn movement(
    mut query: Query<(&mut Instance,)>,
    mut app: ResMut<App>,
    obb: Res<OrientedBoundingBox>
) {
    let mut instances = vec![];
    let mut temp_instance = Instance {
        ..Default::default()
    };
    for (mut instance,) in &mut query {
        instance.position[0] += 10. * delta_time_to_seconds(app.dt);
        let instance_raw = instance.to_raw();
        if instance_raw.is_some() {
            instances.push(instance_raw.unwrap());
        }
        temp_instance = *instance;
    }
    obb.check_collision_with_ray(ray_origin, app.window_events.mouse_dir_ray, &temp_instance);
    temp_instance.update(instances, &mut app.asset_server);
}