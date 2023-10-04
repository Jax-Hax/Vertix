use glam::{Quat, Vec3};
use hecs::World;
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
};
use winit::event::WindowEvent;

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
    let instances = (0..NUM_INSTANCES_PER_ROW)
        .flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = Vec3 { x, y: 0.0, z };

                let rotation = Quat::from_axis_angle(position.normalize(), f32::to_radians(45.0));

                (Instance { position, rotation, ..Default::default() },)
            })
        })
        .collect::<Vec<_>>();
    state.world.spawn_batch(instances);
    let instances_raw = update_instances(&mut state.world);
    let container = state
        .create_model_instances("cube.obj", instances_raw, true)
        .await;
    state.world.spawn((container,));
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
    for (_entity, (instance,)) in state
        .world
        .query_mut::<(&mut Instance,)>()
    {
        instance.position[0] += 0.01;
    }
    let instances = update_instances(&mut state.world);
    for (_entity, (game_object,)) in state
        .world //possibly do something with querying via entities and storing the entity id with query_one, which should prob have a mainstate then
        .query_one_mut::<(&mut InstanceContainer,)>()
    {
        game_object.update(instances, &state.queue);
    }
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