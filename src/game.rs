use crate::{color::Color, math::rotor::Rotor};
use anyhow::{bail, Context};
use encase::{ShaderSize, ShaderType, StorageBuffer, UniformBuffer};
use std::{sync::Arc, time::Duration};
use winit::{
    dpi::PhysicalSize,
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[derive(ShaderType)]
struct Camera {
    transform: Rotor,
    v_fov: f32,
}

const CHUNK_SIZE: usize = 4;

#[derive(ShaderType)]
struct Block {
    color: Color,
    exists: u32,
}

#[derive(ShaderType)]
struct Chunk {
    data: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
}

pub struct Game {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    main_texture: wgpu::Texture,
    main_texture_bind_group_layout: wgpu::BindGroupLayout,
    main_texture_bind_group: wgpu::BindGroup,
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    chunk_storage_buffer: wgpu::Buffer,
    chunk_bind_group: wgpu::BindGroup,
    compute_pipeline: wgpu::ComputePipeline,

    movement_state: MovementState,
    camera_transform: Rotor,
    camera_vertical_look: Rotor,
    chunk: Chunk,
}

impl Game {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .context("Could not find an adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let PhysicalSize { width, height } = window.inner_size();
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_DST,
            format: wgpu::TextureFormat::Rgba8Unorm,
            width,
            height,
            present_mode: surface_capabilities
                .present_modes
                .iter()
                .copied()
                .find(|present_mode| matches!(present_mode, wgpu::PresentMode::Mailbox))
                .unwrap_or(wgpu::PresentMode::AutoNoVsync),
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_configuration);

        let main_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Main Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_configuration.format,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let main_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Main Texture Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: main_texture.format(),
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });
        let main_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main Texture Bind Group"),
            layout: &main_texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &main_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            }],
        });

        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: Camera::SHADER_SIZE.get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(Camera::SHADER_SIZE),
                    },
                    count: None,
                }],
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        let chunk_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Chunk Storage Buffer"),
            size: Chunk::SHADER_SIZE.get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let chunk_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Chunk Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: Some(Chunk::SHADER_SIZE),
                    },
                    count: None,
                }],
            });
        let chunk_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Chunk Bind Group"),
            layout: &chunk_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: chunk_storage_buffer.as_entire_binding(),
            }],
        });

        let compute_shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl"));
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[
                    &main_texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &chunk_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        Ok(Game {
            window,
            device,
            queue,
            surface_configuration,
            surface,
            main_texture,
            main_texture_bind_group_layout,
            main_texture_bind_group,
            camera_uniform_buffer,
            camera_bind_group,
            chunk_storage_buffer,
            chunk_bind_group,
            compute_pipeline,

            movement_state: MovementState::default(),
            camera_transform: Rotor::translation([-4.5, 0.5, -1.5, 0.5]),
            camera_vertical_look: Rotor::IDENTITY,
            chunk: Chunk {
                data: std::array::from_fn(|i| {
                    if i % 3 == 0 {
                        Block {
                            color: Color {
                                r: 0.0,
                                g: 1.0,
                                b: 0.0,
                            },
                            exists: 1,
                        }
                    } else {
                        Block {
                            color: Color {
                                r: 1.0,
                                g: 0.0,
                                b: 0.0,
                            },
                            exists: 0,
                        }
                    }
                }),
            },
        })
    }

    pub fn keyboard(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        let value = if key_event.state.is_pressed() {
            1.0
        } else {
            0.0
        };
        match key_event.physical_key {
            PhysicalKey::Code(key_code) => match key_code {
                KeyCode::KeyS => self.movement_state.backward = value,
                KeyCode::KeyW => self.movement_state.forward = value,
                KeyCode::KeyA => self.movement_state.left = value,
                KeyCode::KeyD => self.movement_state.right = value,
                KeyCode::ShiftLeft => self.movement_state.down = value,
                KeyCode::Space => self.movement_state.up = value,
                _ => {}
            },
            PhysicalKey::Unidentified(_) => {}
        }
        Ok(())
    }

    pub fn cursor(&mut self, x: f32, y: f32) -> anyhow::Result<()> {
        self.camera_vertical_look = self.camera_vertical_look * Rotor::rotation_xy(y * -0.001);
        self.camera_transform = self.camera_transform * Rotor::rotation_xz(x * 0.001);
        Ok(())
    }

    pub fn scroll(&mut self, _x: f32, y: f32) -> anyhow::Result<()> {
        self.camera_transform = self.camera_transform * Rotor::rotation_xw(y * 0.01);
        Ok(())
    }

    pub fn update(&mut self, dt: Duration) -> anyhow::Result<()> {
        self.camera_transform =
            self.camera_transform * self.movement_state.transform(dt.as_secs_f32());
        Ok(())
    }

    pub fn fixed_update(&mut self, _ts: Duration) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if width > 0 && height > 0 {
            self.surface_configuration.width = width;
            self.surface_configuration.height = height;
            self.surface
                .configure(&self.device, &self.surface_configuration);

            self.main_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Main Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.surface_configuration.format,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            self.main_texture_bind_group =
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Main Texture Bind Group"),
                    layout: &self.main_texture_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &self
                                .main_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    }],
                });
        }
        Ok(())
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        let texture = loop {
            match self.surface.get_current_texture() {
                Ok(texture) => break texture,
                Err(e @ wgpu::SurfaceError::Timeout) => {
                    eprintln!("{e}");
                    return Ok(());
                }
                Err(wgpu::SurfaceError::Outdated) | Err(wgpu::SurfaceError::Lost) => {
                    let PhysicalSize { width, height } = self.window.inner_size();
                    if width == 0 || height == 0 {
                        return Ok(());
                    }
                    self.resize(width, height)?;
                }
                Err(e @ wgpu::SurfaceError::OutOfMemory) => bail!(e),
            }
        };

        {
            let mut buffer = UniformBuffer::new([0; Camera::SHADER_SIZE.get() as _]);
            buffer.write(&Camera {
                transform: self.camera_transform * self.camera_vertical_look,
                v_fov: 90.0f32.to_radians(),
            })?;
            self.queue
                .write_buffer(&self.camera_uniform_buffer, 0, &buffer.into_inner());
        }

        {
            let mut buffer = StorageBuffer::new([0; Chunk::SHADER_SIZE.get() as _]);
            buffer.write(&self.chunk)?;
            self.queue
                .write_buffer(&self.chunk_storage_buffer, 0, &buffer.into_inner());
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        {
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Ray Tracing Compute Pass"),
                    timestamp_writes: None,
                });

                compute_pass.set_pipeline(&self.compute_pipeline);
                compute_pass.set_bind_group(0, &self.main_texture_bind_group, &[]);
                compute_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                compute_pass.set_bind_group(2, &self.chunk_bind_group, &[]);
                compute_pass.dispatch_workgroups(
                    self.main_texture.size().width.div_ceil(16),
                    self.main_texture.size().height.div_ceil(16),
                    1,
                );
            }
            encoder.copy_texture_to_texture(
                self.main_texture.as_image_copy(),
                texture.texture.as_image_copy(),
                self.main_texture.size(),
            );
        }
        self.queue.submit([encoder.finish()]);

        self.window.pre_present_notify();
        texture.present();
        Ok(())
    }
}

#[derive(Default)]
struct MovementState {
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
    up: f32,
    down: f32,
}

impl MovementState {
    fn transform(&self, dt: f32) -> Rotor {
        Rotor::translation(
            [
                self.forward - self.backward,
                self.up - self.down,
                self.right - self.left,
                0.0,
            ]
            .map(|m| m * 5.0 * dt),
        )
    }
}
