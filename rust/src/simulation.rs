use crate::types::*;
use crate::sph::*;
use crate::physics::*;

pub fn simulation_step(particle_data: &mut ParticleData, smoothing_radius: f64, window_size: [f64; 2]) {
    predict_positions(particle_data);
    precalculate_densities(particle_data, smoothing_radius);
    apply_pressure_forces(particle_data, smoothing_radius);
    //apply_gravity(&mut particle_data.velocities);
    apply_friction(&mut particle_data.velocities);
    resolve_collisions(particle_data, window_size);
    update_positions(particle_data);
}
