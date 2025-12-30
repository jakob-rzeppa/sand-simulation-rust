use wgpu;

pub struct ParticleBuffers {
    pub particle_grid_buffer: wgpu::Buffer,
    pub grid_dims_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub grid_width: u32,
    pub grid_height: u32,
}

impl ParticleBuffers {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        initial_particle_grid: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Self {
        // Create particle buffer
        // Each particle is 1 byte (u8)
        let buffer_size = (initial_particle_grid.len()) as u64;
        let particle_grid_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Particle Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write initial particle data to buffer
        queue.write_buffer(
            &particle_grid_buffer,
            0,
            &initial_particle_grid, // cast not needed, since already &[u8]
        );

        // Create grid dimensions uniform buffer
        let grid_dims_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Grid Dimensions Buffer"),
            size: 8, // 2 * u32
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write grid dimensions to buffer
        queue.write_buffer(&grid_dims_buffer, 0, bytemuck::cast_slice(&[width, height]));

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Particle Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_grid_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: grid_dims_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            particle_grid_buffer,
            grid_dims_buffer,
            bind_group,
            bind_group_layout,
            grid_width: width,
            grid_height: height,
        }
    }
}
