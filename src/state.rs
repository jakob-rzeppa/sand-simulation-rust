use crate::particle::Particle;
use std::sync::Arc;
use winit::window::Window;

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
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
        // The instance is a handle to our GPU -> its main purpose is to create the surface and adapter
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // The surface is the part of the window to draw to.
        let surface = instance.create_surface(window.clone())?;

        // The adapter is a handle for our actual graphics card.
        // You can use this to get information about the graphics card.
        // WARN: The options passed to request_adapter aren't guaranteed to work for all devices.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                // The force_fallback_adapter forces wgpu to pick an adapter that will work on all hardware.
                // This usually means that the rendering backend will use a "software" system instead of hardware such as a GPU.
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let texture_extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // Create texture to hold particles
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Particle Texture"),
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            texture,
            texture_extent,
            particle_grid,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
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
        self.queue.write_texture(
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
        let output = self.surface.get_current_texture()?;

        // Create a CommandEncoder to create the actual commands to send to the GPU.
        // The encoder builds a command buffer that we can then send to the GPU.
        let mut encoder = self
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
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
