use bevy_ecs::system::{Query, Res, ResMut};
use glam::{Quat, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
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
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    const SPACE_BETWEEN: f32 = 3.0;
    const NUM_INSTANCES_PER_ROW: usize = 100;
    let mut instances = vec![];
    for x in 0..NUM_INSTANCES_PER_ROW {
        for y in 0..NUM_INSTANCES_PER_ROW {
            let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
            let y = SPACE_BETWEEN * (y as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

            let position = Vec3 { x, y, z: 0. };

            let rotation = Quat::from_axis_angle(position.normalize(), f32::to_radians(45.0));

            let instance = Instance {
                position,
                rotation,
                ..Default::default()
            };

            instances.push((instance,));
        }
    }
    state
        .create_model_instances(
            "cube.obj",
            instances.iter_mut().map(|(instance,)| instance).collect(),
            true,
        )
        .await;
    state.world.spawn_batch(instances);
    state.schedule.add_systems((movement, movement_with_key));
    //render loop
    run_event_loop(state, event_loop, Some(default_3d_cam));
}
fn movement(
    mut query: Query<(&mut Instance,)>,
    mut instance_update: ResMut<UpdateInstance>,
    delta_time: Res<DeltaTime>,
) {
    let mut instances = vec![];
    let mut temp_instance = Instance {
        ..Default::default()
    };
    for (mut instance,) in &mut query {
        instance.position[0] += 10. * delta_time_to_seconds(delta_time.dt);
        let instance_raw = instance.to_raw();
        if instance_raw.is_some() {
            instances.push(instance_raw.unwrap());
        }
        temp_instance = *instance;
    }
    temp_instance.update(instances, &mut instance_update);
}
fn movement_with_key(
    mut query: Query<(&mut Instance,)>,
    mut instance_update: ResMut<UpdateInstance>,
    delta_time: Res<DeltaTime>,
    window_events: Res<WindowEvents>,
) {
    if window_events.is_key_pressed(VirtualKeyCode::D, None) {
        let mut instances = vec![];
        let mut temp_instance = Instance {
            ..Default::default()
        };
        for (mut instance,) in &mut query {
            instance.position[1] += 50. * delta_time_to_seconds(delta_time.dt);
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instances.push(instance_raw.unwrap());
            }
            temp_instance = *instance;
        }
        temp_instance.update(instances, &mut instance_update);
    }
}
