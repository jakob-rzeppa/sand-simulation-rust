use crate::particle::Particle;

/// Manages the texture that holds particle data for rendering
pub struct TextureManager {
    texture: wgpu::Texture,
    extent: wgpu::Extent3d,
}

impl TextureManager {
    /// Creates a new texture manager with the specified dimensions
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Particle Texture"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        Self { texture, extent }
    }

    /// Updates the texture with particle data
    pub fn update_from_particles(&self, queue: &wgpu::Queue, particles: &[Particle]) {
        // Convert particles to BGRA bytes (to match surface format)
        let particle_data: Vec<u8> = particles.iter().flat_map(|p| p.to_bgra()).collect();

        // Write particle data to texture
        queue.write_texture(
            self.texture.as_image_copy(),
            &particle_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.extent.width),
                rows_per_image: Some(self.extent.height),
            },
            self.extent,
        );
    }

    /// Gets the texture reference
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Gets the texture extent
    pub fn extent(&self) -> wgpu::Extent3d {
        self.extent.clone()
    }
}
