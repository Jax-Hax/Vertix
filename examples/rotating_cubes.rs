use glam::{Quat, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
    structs::WorldSpace,
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

                Instance::new(position, rotation)
            })
        })
        .collect::<Vec<_>>();
    let (container, is_dynamic) = state
        .create_model_instances("cube.obj", instances, true)
        .await;
    match is_dynamic {
        Some(_) => state.world.spawn((container, IsDynamic, WorldSpace)),
        None => state.world.spawn((container, WorldSpace)),
    };
    //render loop
    run_event_loop(
        state,
        event_loop,
        Some(update),
        Some(input),
        Some(default_3d_cam),
    );
}

fn update(state: &mut State) {
    for (_entity, (game_object, _)) in state
        .world
        .query_mut::<(&mut InstanceContainer, &IsDynamic)>()
    {
        for instance in &mut game_object.instances {
            instance.position[0] += 0.01;
        }
        game_object.update(&state.queue);
    }
}
fn input(state: &mut State, event: &WindowEvent) {
    //keyboard inputs
    match event {
        WindowEvent::KeyboardInput { input, .. } => match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::F),
                ..
            } => {
                for (_entity, (game_object, _)) in state
                    .world
                    .query_mut::<(&mut InstanceContainer, &IsDynamic)>()
                {
                    for instance in &mut game_object.instances {
                        instance.position[1] += 0.001;
                    }
                    game_object.update(&state.queue);
                }
            }
            _ => {}
        },
        _ => {}
    }
}
