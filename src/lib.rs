pub mod camera;
pub mod structs;
pub mod model;
pub mod loader;
pub mod shader;
pub mod state;
pub mod texture;
pub mod window;
pub mod prefabs;
pub mod shapes;
pub mod event;
pub mod resources;
pub mod instance;
pub mod assets;
pub mod app_resource;
mod render;
pub mod collision {
    pub mod structs_3d;
    pub mod structs_2d;
    pub mod collision_fns_3d;
}
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