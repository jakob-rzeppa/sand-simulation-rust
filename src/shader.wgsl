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

// Mouse position in normalized coordinates (0.0 to 1.0)
@group(0) @binding(2)
var<uniform> mouse_pos: vec2<f32>;

// Selected material type
@group(0) @binding(3)
var<uniform> selected_material: u32;

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

    // Base color based on particle type
    var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);  // Air - white
    
    if (particle_type == 1u) {
        // Sand - sandy color
        color = vec4<f32>(0.76, 0.70, 0.50, 1.0);
    } else if (particle_type == 2u) {
        // Stone - gray color
        color = vec4<f32>(0.57, 0.56, 0.52, 1.0);
    }

    // Make circle around mouse darker (only if mouse is in window)
    // Mouse position is set to negative values when outside window
    if (mouse_pos.x >= 0.0 && mouse_pos.y >= 0.0) {
        let radius = 15.0;
        
        // Convert mouse position from normalized coords to grid coords
        let mouse_grid_x = mouse_pos.x * f32(grid_dims.x);
        let mouse_grid_y = mouse_pos.y * f32(grid_dims.y);
        
        // Calculate distance from current pixel to mouse position
        let dx = f32(pixel_x) - mouse_grid_x;
        let dy = f32(pixel_y) - mouse_grid_y;
        let distance = sqrt(dx * dx + dy * dy);
        
        // If within radius, darken the color
        if (distance <= radius) {
            color = color * 0.7;  // Darken by multiplying by 0.7
        }
    }

    return color;
}
