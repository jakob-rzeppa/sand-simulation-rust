pub fn simulate_particles(particle_grid: &mut [u8], height: u32, width: u32) -> &[u8] {
    let height = height as usize;
    let width = width as usize;

    // Goes from the bottom to the top and simulates each particle
    for y in (0..height - 1).rev() {
        for x in 0..width {
            let idx: usize = (y * width + x) as usize;
            // Sand
            if particle_grid[idx] == 1 {
                // Try to move down
                if particle_grid[idx + width] == 0 {
                    particle_grid[idx] = 0;
                    particle_grid[idx + width] = 1;
                }
                // Try to move down-right
                else if x < width - 1 && particle_grid[idx + width + 1] == 0 {
                    particle_grid[idx] = 0;
                    particle_grid[idx + width + 1] = 1;
                }
                // Try to move down-left
                else if x > 0 && particle_grid[idx + width - 1] == 0 {
                    particle_grid[idx] = 0;
                    particle_grid[idx + width - 1] = 1;
                }
            }
            // Stone
            else if particle_grid[idx] == 2 {
                // Try to move down
                if particle_grid[idx + width] == 0 {
                    particle_grid[idx] = 0;
                    particle_grid[idx + width] = 2;
                }
            }
        }
    }

    particle_grid
}
