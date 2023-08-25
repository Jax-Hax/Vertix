use cgmath::SquareMatrix;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};

use crate::{camera::{self, Camera, CameraController, Projection}, renderer::State, texture};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    // UPDATED!
    pub fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into()
    }
}
pub fn init() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
}

pub fn make_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    (event_loop, window)
}
pub fn make_camera(
    device: Device,
    config: SurfaceConfiguration,
) -> (Camera, CameraController, Buffer, BindGroup, CameraUniform,Projection) {
    let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let projection =
        camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
    let camera_controller = camera::CameraController::new(4.0, 0.4);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, &projection);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });
    (
        camera,
        camera_controller,
        camera_buffer,
        camera_bind_group,
        camera_uniform,
        projection,
    )
}

pub fn resize(mut state: State, new_size: winit::dpi::PhysicalSize<u32>, mut projection: Projection) -> (State, Projection){
    if new_size.width > 0 && new_size.height > 0 {
        projection.resize(new_size.width, new_size.height);
        state.size = new_size;
        state.config.width = new_size.width;
        state.config.height = new_size.height;
        state.surface.configure(&state.device, &state.config);
        state.depth_texture =
            texture::Texture::create_depth_texture(&state.device, &state.config, "depth_texture");
    }
    (state, projection)
}
