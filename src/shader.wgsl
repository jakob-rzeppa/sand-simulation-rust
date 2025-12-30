// Vertex shader outputs
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

// Particle buffer (each particle is 1 byte - u8)
@group(0) @binding(0)
var<storage, read> particles: array<u32>;

// Uniforms for grid dimensions
@group(0) @binding(1)
var<uniform> grid_dims: vec2<u32>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Full-screen triangle
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    
    out.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, y);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate pixel coordinates
    let pixel_x = u32(in.tex_coords.x * f32(grid_dims.x));
    let pixel_y = u32(in.tex_coords.y * f32(grid_dims.y));
    
    // Get particle index
    let index = pixel_y * grid_dims.x + pixel_x;
    
    // Read particle type from buffer (4 bytes at a time, extract the one we need)
    let word_index = index / 4u;
    let byte_offset = index % 4u;
    let word = particles[word_index];
    
    // Extract the byte we need
    let particle_type = (word >> (byte_offset * 8u)) & 0xFFu;
    
    // Return color based on particle type
    if (particle_type == 0u) {
        // Air - white
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    } else {
        // Sand - sandy color
        return vec4<f32>(0.76, 0.70, 0.50, 1.0);
    }
}
