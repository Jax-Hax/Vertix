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
    let mut instances_ref = vec![];
    let mut instances = vec![];
    for x in 0..NUM_INSTANCES_PER_ROW {
        for y in 0..NUM_INSTANCES_PER_ROW {
            for z in 0..NUM_INSTANCES_PER_ROW {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let y = SPACE_BETWEEN * (y as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = Vec3 { x, y, z };

                let rotation = Quat::from_axis_angle(position.normalize(), f32::to_radians(45.0));

                let mut instance = Instance { position, rotation, ..Default::default() };
                
                instances.push((instance,));
                instances_ref.push(instances.last().unwrap().0);
            }
        }
    }
    state
        .create_model_instances("cube.obj", instances_ref, true)
        .await;
    state.world.spawn_batch(instances);
    //render loop
    run_event_loop(
        state,
        event_loop,
        Some(update),
        None,
        Some(default_3d_cam),
    );
}

fn update(state: &mut State) {
    let mut instances = vec![];
    let mut temp_instance;
    for (_entity, (instance,)) in state
        .world
        .query_mut::<(&mut Instance,)>()
    {
        instance.position[0] += 0.01;
        instances.push(instance.to_raw());
        temp_instance = instance;
    }
    temp_instance.update(instances, state);
}