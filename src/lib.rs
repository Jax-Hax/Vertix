pub mod camera;
pub mod engine;
pub mod model;
pub mod resources;
pub mod shader;
pub mod state;
pub mod texture;
pub mod window;
pub mod prelude {
    pub use crate::{
        engine::{Instance,InstanceContainer, IsDynamic},
        state::{run_event_loop, State},
        model::Vertex
    };
    pub use cgmath::prelude::*;
    #[cfg(target_arch = "wasm32")]
    pub use wasm_bindgen::prelude::*;
    pub use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
}