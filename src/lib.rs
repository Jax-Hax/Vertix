use crate::{
    engine::Instance,
    state::{run_event_loop, State},
};
use cgmath::prelude::*;
use engine::{GameObject, GameObjectType};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

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
    let mut entities = vec![];
    let instances = state
        .create_dynamic_instances("cube.obj", "cube", instances)
        .await;
    entities.push(instances);

    //render loop
    run_event_loop(state, event_loop, update, keyboard_input, entities);
}
fn update(state: &mut State, entities: &mut Vec<GameObject>) {
    if let GameObjectType::DynamicMesh(ref mut instances) = &mut entities[0].object_type {
        for instance in &mut instances.instances {
            instance.position[0] += 0.01;
        }
        state.update_instances(&instances);
    }
}
fn keyboard_input(state: &mut State, entities: &mut Vec<GameObject>, event: &KeyboardInput) {
    //keyboard inputs
    match event {
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::F),
            ..
        } => if let GameObjectType::DynamicMesh(ref mut instances) = &mut entities[0].object_type {
            for instance in &mut instances.instances {
                instance.position[1] += 0.01;
            }
            state.update_instances(&instances);
        },
        _ => {}
    }
}
