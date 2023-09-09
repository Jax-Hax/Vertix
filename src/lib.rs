use crate::{
    engine::Instance,
    state::{run_event_loop, State},
};
use hecs::*;
use cgmath::prelude::*;
use engine::{InstanceContainer, IsDynamic};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

mod camera;
mod engine;
mod model;
mod resources;
mod shader;
mod state;
mod texture;
mod window;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true).await;
    //add models
    const SPACE_BETWEEN: f32 = 3.0;
    const NUM_INSTANCES_PER_ROW: usize = 10;
    let instances = (0..NUM_INSTANCES_PER_ROW)
        .flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance { position, rotation }
            })
        })
        .collect::<Vec<_>>();
    state
        .create_dynamic_instances("cube.obj", instances)
        .await;

    //render loop
    run_event_loop(state, event_loop, update, keyboard_input);
}
fn update(state: &mut State) {
    for (_entity, (game_object, _)) in state.world.query_mut::<(&mut InstanceContainer, &IsDynamic)>() {
        println!("yeah");
        for instance in &mut game_object.instances {
            instance.position[0] += 0.001;
        }
        game_object.update(&state.queue);
    }
}
fn keyboard_input(state: &mut State, event: &KeyboardInput) {
    //keyboard inputs
    match event {
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::F),
            ..
        } => 
        for (_entity, (game_object,_)) in state.world.query_mut::<(&mut InstanceContainer,&IsDynamic)>() {
            for instance in &mut game_object.instances {
                instance.position[1] += 0.001;
            }
            game_object.update(&state.queue);
        },
        _ => {}
    }
}
