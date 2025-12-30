use crate::gpu_context::GpuContext;
use crate::particle::Particle;
use crate::texture_manager::TextureManager;
use std::sync::Arc;
use winit::window::Window;

pub struct State {
    gpu_context: GpuContext,
    is_surface_configured: bool,
    pub window: Arc<Window>,
    texture_manager: TextureManager,
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

        // Create texture manager handling the texture
        let texture_manager = TextureManager::new(
            &gpu_context.device,
            gpu_context.surface_format,
            width,
            height,
        );

        Ok(Self {
            gpu_context,
            is_surface_configured: false,
            window,
            texture_manager,
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

        self.texture_manager
            .update_from_particles(&self.gpu_context.queue, &self.particle_grid);

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
            self.texture_manager.texture().as_image_copy(),
            output.texture.as_image_copy(),
            self.texture_manager.extent(),
        );

        // submit will accept anything that implements IntoIter
        self.gpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
