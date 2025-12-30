pub fn simulate_particles(particle_grid: &mut [u8], height: u32, width: u32) -> &[u8] {
    let height = height as usize;
    let width = width as usize;

    for y in (0..height - 1).rev() {
        for x in 0..width {
            let idx: usize = (y * width + x) as usize;
            if particle_grid[idx] == 1 && particle_grid[idx + width] == 0 {
                particle_grid[idx] = 0;
                particle_grid[idx + width] = 1;
            }
        }
    }

    particle_grid
}
