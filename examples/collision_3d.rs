use bevy_ecs::system::{Query, ResMut};
use glam::Vec3;
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*, app_resource::App, collision::structs_3d::OrientedBoundingBox,
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
    mut query: Query<(&mut Instance,&mut OrientedBoundingBox)>,
    mut app: ResMut<App>,
) {
    let mut instances = vec![];
    let mut temp_instance = Instance {
        ..Default::default()
    };
    for (instance,obb) in &mut query {
        let instance_raw = instance.to_raw();
        if instance_raw.is_some() {
            let (is_collided,_collision_dist) = obb.check_collision_with_ray(app.camera.camera_transform.position, app.window_events.mouse_ray_direction, &temp_instance);
            if is_collided {
                println!("trueee");
            }
            else {
                println!("not truee");
            }
            instances.push(instance_raw.unwrap());
        }
        temp_instance = *instance;
    }
    
    temp_instance.update(instances, &mut app.asset_server);
}