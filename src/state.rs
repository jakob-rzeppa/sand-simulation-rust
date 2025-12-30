use crate::buffers::ParticleBuffers;
use crate::gpu_context::GpuContext;
use crate::simulate::simulate_particles;
use crate::MS_PER_SIMULATION;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::window::Window;

pub struct State {
    pub window: Arc<Window>,

    gpu_context: GpuContext,
    is_surface_configured: bool,

    render_pipeline: wgpu::RenderPipeline,

    particle_buffers: ParticleBuffers,
    particle_grid: Vec<u8>,

    last_update: Instant,
    update_interval: Duration,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        initial_particle_grid: Vec<u8>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        // Create gpu context containing the gpu instance, adapter, surface, device, queue, surface format and surface config
        let gpu_context = GpuContext::new(window.clone()).await?;

        // Create particle buffers and bind group
        let particle_buffers = ParticleBuffers::new(
            &gpu_context.device,
            &gpu_context.queue,
            initial_particle_grid.clone(),
            width,
            height,
        );

        // Load shader
        let shader = gpu_context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Particle Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        // Create render pipeline layout
        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&particle_buffers.bind_group_layout],
                    immediate_size: 0,
                });

        // Create render pipeline
        let render_pipeline =
            gpu_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: gpu_context.surface_format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: Default::default(),
                    }),
                    // The primitive field describes how to interpret our vertices when converting them into triangles.
                    primitive: wgpu::PrimitiveState {
                        // every three vertices will correspond to one triangle
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview_mask: None,
                    cache: None,
                });

        Ok(Self {
            gpu_context,
            is_surface_configured: false,
            window,
            render_pipeline,
            particle_buffers,
            particle_grid: initial_particle_grid,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(MS_PER_SIMULATION),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu_context.surface_config.width = width;
            self.gpu_context.surface_config.height = height;
            self.gpu_context
                .surface
                .configure(&self.gpu_context.device, &self.gpu_context.surface_config);
            self.is_surface_configured = true;
        }
    }

    pub fn set_simulation_speed(&mut self, updates_per_second: u32) {
        self.update_interval = Duration::from_secs_f32(1.0 / updates_per_second as f32);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // Only simulate if enough time has passed
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            let updated_data = simulate_particles(
                &mut self.particle_grid,
                self.particle_buffers.grid_height,
                self.particle_buffers.grid_width,
            );

            // Update GPU buffer with simulated data
            self.gpu_context.queue.write_buffer(
                &self.particle_buffers.particle_grid_buffer,
                0,
                updated_data,
            );

            self.last_update = now;
        }

        // Get the current surface texture
        let output = self.gpu_context.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a CommandEncoder
        let mut encoder =
            self.gpu_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Create render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.particle_buffers.bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Draw full-screen triangle
        }

        // Submit commands and present
        self.gpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
