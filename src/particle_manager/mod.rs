use crate::RADIUS_ADD_PARTICLES;
use simulate::simulate_particles;
use winit::dpi::PhysicalSize;

mod simulate;

pub struct ParticleManager {
    particle_grid: Vec<u8>,
    width: u32,
    height: u32,

    // the currently selected material to be created when clicking
    selected_material: u8,
}

impl ParticleManager {
    pub fn new(width: u32, height: u32) -> Self {
        let particle_grid = vec![0; width as usize * height as usize];

        Self {
            particle_grid,
            width,
            height,

            // Initialize to sand
            selected_material: 1,
        }
    }

    pub fn cycle_material_up(&mut self) {
        // Cycle through materials: 0 (air) -> 1 (sand) -> 2 (stone) -> 0
        self.selected_material = (self.selected_material + 1) % 3;
    }

    pub fn cycle_material_down(&mut self) {
        // Cycle backwards: 0 (air) -> 2 (stone) -> 1 (sand) -> 0
        self.selected_material = if self.selected_material == 0 {
            2
        } else {
            self.selected_material - 1
        };
    }

    pub fn add_material_at_cursor(
        &mut self,
        window_size: &PhysicalSize<u32>,
        cursor_x: f64,
        cursor_y: f64,
    ) {
        // Convert cursor position to grid coordinates
        let grid_x = ((cursor_x / window_size.width as f64) * self.width as f64) as i32;
        let grid_y = ((cursor_y / window_size.height as f64) * self.height as f64) as i32;

        let radius = RADIUS_ADD_PARTICLES as i32;

        // Add the selected material in a circle around the cursor
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                // Check if point is within circle
                if dx * dx + dy * dy <= radius * radius {
                    let x = grid_x + dx;
                    let y = grid_y + dy;

                    // Check bounds
                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        let index = (y * self.width as i32 + x) as usize;
                        self.particle_grid[index] = self.selected_material;
                    }
                }
            }
        }
    }

    pub fn simulate_particles(&mut self) {
        simulate_particles(&mut self.particle_grid, self.height, self.width);
    }

    pub fn particle_grid(&self) -> &[u8] {
        self.particle_grid.as_slice()
    }

    pub fn selected_material(&self) -> u8 {
        self.selected_material
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
