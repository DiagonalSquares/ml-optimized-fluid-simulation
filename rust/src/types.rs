#[repr(C)]
pub struct ParticleData {
    pub mass: f64,
    pub positions: Vec<[f64; 2]>,
    pub predicted_positions: Vec<[f64; 2]>,
    pub velocities: Vec<[f64; 2]>,
    pub densities: Vec<f64>,
}