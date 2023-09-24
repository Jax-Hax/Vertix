pub mod camera;
pub mod structs;
pub mod model;
pub mod resources;
pub mod shader;
pub mod state;
pub mod texture;
pub mod window;
pub mod prelude {
    pub use crate::{
        structs::{Instance,InstanceContainer, IsDynamic, WorldSpace},
        state::{run_event_loop, State},
        model::Vertex,
        camera::Camera
    };
    #[cfg(target_arch = "wasm32")]
    pub use wasm_bindgen::prelude::*;
    pub use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
}