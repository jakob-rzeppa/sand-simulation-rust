use std::sync::Arc;
use winit::window::Window;

/// Handles GPU initialization and context management
pub struct GpuContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    /// Creates a new GPU context by initializing the instance, adapter, surface, device, queue, surface format and surface config
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
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
        // Shader code assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            surface_format,
            surface_config,
        })
    }
}
