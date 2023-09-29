use wgpu::{Adapter, Surface};
use winit::{
    dpi::{PhysicalSize, PhysicalPosition},
    event_loop::EventLoop,
    window::{Fullscreen, WindowBuilder},
};
pub struct Window {
    pub window: winit::window::Window,
    pub size: PhysicalSize<u32>,
    pub adapter: Adapter,
    pub surface: Surface,
}
impl Window {
    pub async fn new(mouse_lock: bool) -> (Self, EventLoop<()>) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
            } else {
                env_logger::init();
            }
        }

        let event_loop = EventLoop::new();
        let monitor = event_loop.primary_monitor().unwrap();
        let video_mode = monitor.video_modes().next();
        let size = video_mode
            .clone()
            .map_or(PhysicalSize::new(800, 600), |vm| vm.size());
        let window = WindowBuilder::new()
            .with_title("WGPUCraft")
            .with_fullscreen(video_mode.map(|vm| Fullscreen::Exclusive(vm)))
            .build(&event_loop)
            .unwrap();

        if window.fullscreen().is_none() {
            window.set_inner_size(PhysicalSize::new(512, 512));
        }
        if mouse_lock {
            window.set_cursor_visible(false);
        }
        window.set_visible(true);
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;

                    // Request fullscreen, if denied, continue as normal
                    match canvas.request_fullscreen() {
                        Ok(_) => {}
                        Err(_) => (),
                    }
                    if mouse_lock {
                        let canvas_two = canvas.clone();
                        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                            // Request pointer lock
                            canvas_two.request_pointer_lock();

                            // Handle other mouse events here
                        })
                            as Box<dyn FnMut(_)>);
                    }
                    // Attach the event handler to the canvas
                    canvas
                        .add_event_listener_with_callback(
                            "mousedown",
                            closure.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add event listener");

                    closure.forget();

                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        log::warn!("WGPU setup");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        window.set_visible(true);
        if mouse_lock {
            #[cfg(any(target_arch = "wasm32", target_os = "macos"))]
            {
                match window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
                    Ok(()) => {}
                    Err(error) => {
                        println!("Error occurred: {:?}", error);
                    }
                }
            }
            #[cfg(not(any(target_arch = "wasm32", target_os = "macos")))]
            {
                match window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                    Ok(()) => {}
                    Err(error) => {
                        println!("Error occurred: {:?}", error);
                    }
                }
            }
        }

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        log::warn!("device and queue");
        (
            Self {
                window,
                size,
                adapter,
                surface,
            },
            event_loop,
        )
    }
    pub fn normalize_position(&self, pos: &PhysicalPosition<f64>) -> PhysicalPosition<f32>{ 
        let normalized_x = (pos.x as f32/ self.size.width as f32) * 2. - 1.; //normalize to be between -1 and 1 instead of 0-1
        let normalized_y = ((pos.y as f32 / self.size.height as f32) - 0.5) * -2.; //normalize to be between -1 and 1 as well as flip
        PhysicalPosition { x: normalized_x, y: normalized_y }
    }
}
