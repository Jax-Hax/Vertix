pub mod camera;
pub mod structs;
pub mod model;
pub mod loader;
pub mod shader;
pub mod state;
pub mod texture;
pub mod window;
pub mod prefabs;
pub mod primitives;
pub mod collision;
pub mod event;
pub mod resources;
pub mod instance;
mod render;
pub mod prelude {
    pub use crate::{
        instance::{Instance,InstanceRaw},
        event::{run_event_loop,delta_time_to_seconds},
        state::State,
        structs::Vertex,
        camera::Camera,
        resources::*
    };
    #[cfg(target_arch = "wasm32")]
    pub use wasm_bindgen::prelude::*;
    pub use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
}