use crate::{
    assets::AssetServer,
    camera::{Camera, CameraStruct},
    resources::{MouseClickType, WindowEvents},
    shader,
    structs::CameraController,
    texture, window, app_resource::App,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use instant::Duration;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
pub struct State {
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub depth_texture: texture::Texture,
    pub window: window::Window,
    pub mouse_locked: bool,
    pub world: World,
    pub schedule: Schedule,
}

impl State {
    pub async fn new(
        mouse_lock: bool,
        build_path: &str,
        cam: Camera,
        speed: f32,
        sensitivity: f32,
    ) -> (Self, EventLoop<()>) {
        let (window, event_loop) = window::Window::new(mouse_lock).await;
        let (device, queue) = window
            .adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();

        log::warn!("Surface");
        let surface_caps = window.surface.get_capabilities(&window.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.size.width,
            height: window.size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        window.surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        //camera
        let camera = CameraStruct::new(
            &device,
            &config,
            cam,
            CameraController::new(speed, sensitivity),
        );

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = shader::make_shader(
            include_str!("shader.wgsl"),
            &device,
            render_pipeline_layout,
            &config,
        );
        window.window.set_visible(true);
        let mut world = World::new();
        let asset_server = AssetServer::new(
            device,
            queue,
            build_path.to_string(),
            texture_bind_group_layout,
        );
        let mut window_events = WindowEvents {
            keys_pressed: vec![],
            screen_mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 },
            world_mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 },
            left_mouse: MouseClickType::NotHeld,
            right_mouse: MouseClickType::NotHeld,
            middle_mouse: MouseClickType::NotHeld,
            aspect_ratio: (config.width as f32) / (config.height as f32),
            mouse_ray_direction: Vec3::ZERO,
        };
        window_events.calculate_mouse_dir(&camera.projection, &camera.camera_uniform.view_proj);
        world.insert_resource(App {asset_server, dt: Duration::ZERO,window_events, camera});
        let schedule = Schedule::default();
        (
            Self {
                config,
                render_pipeline,
                depth_texture,
                window,
                mouse_locked: mouse_lock,
                world,
                schedule,
            },
            event_loop,
        )
    }
    pub fn window(&self) -> &Window {
        &self.window.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let mut app = self.world
                .get_resource_mut::<App>()
                .unwrap();
            app.camera
                .projection
                .resize(new_size.width, new_size.height);
            app.window_events
                .update_aspect_ratio(new_size.width, new_size.height);
            self.window.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            let device = &app.asset_server.device;
            self.window.surface.configure(device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&device, &self.config, "depth_texture");
        }
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let key_pressed = (*key, *state);
                let mut app = self.world
                    .get_resource_mut::<App>()
                    .unwrap();
                app.window_events
                    .keys_pressed
                    .push(key_pressed);
                app.camera.camera_controller.process_keyboard(*key, *state)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let mut app = self.world
                    .get_resource_mut::<App>()
                    .unwrap();
                app.camera.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let mut app = self.world
                    .get_resource_mut::<App>()
                    .unwrap();
                let mut events = &mut app.window_events;
                match button {
                    MouseButton::Left => {
                        events.left_mouse = if *state == ElementState::Pressed {
                            MouseClickType::Clicked
                        } else {
                            MouseClickType::Released
                        }
                    }
                    MouseButton::Right => {
                        events.right_mouse = if *state == ElementState::Pressed {
                            MouseClickType::Clicked
                        } else {
                            MouseClickType::Released
                        }
                    }
                    MouseButton::Middle => {
                        events.middle_mouse = if *state == ElementState::Pressed {
                            MouseClickType::Clicked
                        } else {
                            MouseClickType::Released
                        }
                    }
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }
    pub fn update(&mut self) {
        let mut app = self.world
                    .get_resource_mut::<App>()
                    .unwrap();
        app.camera.update_view_proj();
        app.asset_server.queue.write_buffer(
            &app.camera.buffer,
            0,
            bytemuck::cast_slice(&[app.camera.camera_uniform]),
        );
    }
}
