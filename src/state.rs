use crate::gpu_context::GpuContext;
use crate::particle::Particle;
use std::sync::Arc;
use winit::window::Window;

pub struct State {
    gpu_context: GpuContext,
    is_surface_configured: bool,
    pub(crate) window: Arc<Window>,
    texture: wgpu::Texture,
    texture_extent: wgpu::Extent3d,
    particle_grid: Vec<Particle>, // the particles in a grid
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        particle_grid: Vec<Particle>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        // Create gpu context containing the gpu instance, adapter, surface, device, queue, surface format and surface config
        let gpu_context = GpuContext::new(window.clone()).await?;

        let texture_extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // Create texture to hold particles
        let texture = gpu_context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Particle Texture"),
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: gpu_context.surface_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        Ok(Self {
            gpu_context,
            is_surface_configured: false,
            window,
            texture,
            texture_extent,
            particle_grid,
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

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // Convert particles to BGRA bytes (to match surface format)
        let particle_data: Vec<u8> = self
            .particle_grid
            .iter()
            .flat_map(|p| p.to_bgra())
            .collect();

        // Write particle data to texture
        self.gpu_context.queue.write_texture(
            self.texture.as_image_copy(),
            &particle_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Option::from(4 * self.texture_extent.width),
                rows_per_image: Option::from(self.texture_extent.height),
            },
            self.texture_extent,
        );

        // The get_current_texture function will wait for the surface
        // to provide a new SurfaceTexture that we will render to.
        let output = self.gpu_context.surface.get_current_texture()?;

        // Create a CommandEncoder to create the actual commands to send to the GPU.
        // The encoder builds a command buffer that we can then send to the GPU.
        let mut encoder =
            self.gpu_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Copy texture to surface
        encoder.copy_texture_to_texture(
            self.texture.as_image_copy(),
            output.texture.as_image_copy(),
            self.texture_extent,
        );

        // submit will accept anything that implements IntoIter
        self.gpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
