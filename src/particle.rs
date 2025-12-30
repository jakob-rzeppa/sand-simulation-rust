#[derive(Copy, Clone)]
pub enum Particle {
    Air = 0,
    Sand = 1,
}

impl Particle {
    pub fn to_bgra(&self) -> [u8; 4] {
        match self {
            Particle::Air => [255, 255, 255, 255],
            Particle::Sand => [128, 178, 194, 255],
        }
    }
}
