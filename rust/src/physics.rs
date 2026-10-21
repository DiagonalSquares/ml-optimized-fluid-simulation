use rayon::prelude::*;

use crate::types::*;
use crate::sph::*;

const FRICTION: f64 = 0.99;
#[allow(dead_code)]
const GRAVITY: [f64; 2] = [0.0, 0.098];
const TARGET_DENSITY: f64 = 2.0;
const PRESSURE_MULTIPLIER: f64 = 0.5;

pub fn set_positions(positions: &mut Vec<[f64; 2]>, window_size: [f64; 2], randomize: bool) {
    if randomize {
        use rand::Rng;
        let mut rng = rand::rng();
        for position in positions.iter_mut() {
            position[0] = rng.random_range(0.0..window_size[0]);
            position[1] = rng.random_range(0.0..window_size[1]);
        }
    } else {
        let grid_size = positions.len().isqrt();
        let spacing_x = window_size[0] / 2.0 / (grid_size + 1) as f64;
        let spacing_y = window_size[1] / 2.0 / (grid_size + 1) as f64;

        for i in 0..grid_size {
            for j in 0..grid_size {
                let x = spacing_x * (i as f64 + 1.0);
                let y = spacing_y * (j as f64 + 1.0);
                positions[i * grid_size + j] = [x, y];
            }
        }
    }
}

pub fn apply_gravity(velocities: &mut Vec<[f64; 2]>) {
    for velocity in velocities {
        velocity[0] += GRAVITY[0];
        velocity[1] += GRAVITY[1];
    }
}

pub fn apply_friction(velocities: &mut Vec<[f64; 2]>) {
    for velocity in velocities {
        velocity[0] *= FRICTION;
        velocity[1] *= FRICTION;
    }
}

pub fn resolve_collisions(particle_data: &mut ParticleData, window_size: [f64; 2]) {
    for index in 0..particle_data.positions.len() {
        let position = &particle_data.positions[index];
        let velocity = &mut particle_data.velocities[index];

        if position[0] <= 0.0 || position[0] >= window_size[0] {
            velocity[0] = -velocity[0];
        }
        if position[1] <= 0.0 || position[1] >= window_size[1] {
            velocity[1] = -velocity[1];
        }
    }
}

pub fn update_positions(particle_data: &mut ParticleData) {
    for index in 0..particle_data.positions.len() {
        particle_data.positions[index][0] += particle_data.velocities[index][0];
        particle_data.positions[index][1] += particle_data.velocities[index][1];
    }
}

pub fn calculate_pressure_force(particle_data: &ParticleData, index: usize, smoothing_radius: f64) -> [f64; 2] {
    let mut net_force = [0.0, 0.0];
    let reference_position = particle_data.positions[index];

    for i in 0..particle_data.positions.len() {
        if i == index { continue; }

        let distance = ((reference_position[0] - particle_data.positions[i][0]).powf(2.0) +
                             (reference_position[1] - particle_data.positions[i][1]).powf(2.0)).sqrt();

        if distance == 0.0 { continue; }

        let direction: [f64; 2] = [
           (reference_position[0] - particle_data.positions[i][0]) / distance,
           (reference_position[1] - particle_data.positions[i][1]) / distance,
        ];

        let slope = smoothing_kernel_derivative(smoothing_radius, distance);
        let density = particle_data.densities[i];

        if density == 0.0 { continue; }

        let shared_pressure = calculate_shared_pressure(particle_data.densities[index], density);
        net_force[0] += -shared_pressure * direction[0] * slope * particle_data.mass / density;
        net_force[1] += -shared_pressure * direction[1] * slope * particle_data.mass / density;
    }

    return net_force;
}

pub fn apply_pressure_forces(particle_data: &mut ParticleData, smoothing_radius: f64) {
    let pressure_forces: Vec<[f64; 2]> = (0..particle_data.positions.len())
        .into_par_iter()
        .map(|i| calculate_pressure_force(&particle_data, i, smoothing_radius))
        .collect();

        for i in 0..particle_data.velocities.len() {
            if particle_data.densities[i] > 0.0 {
                particle_data.velocities[i][0] += pressure_forces[i][0] / particle_data.densities[i] / particle_data.mass;
                particle_data.velocities[i][1] += pressure_forces[i][1] / particle_data.densities[i] / particle_data.mass;
            }
        }
}

pub fn calculate_shared_pressure(density_a: f64, density_b: f64) -> f64 {
    let pressure_a = convert_density_to_pressure(density_a);
    let pressure_b = convert_density_to_pressure(density_b);
    return (pressure_a + pressure_b) / 2.0;
}

pub fn convert_density_to_pressure(density: f64) -> f64 {
    let density_error = density - TARGET_DENSITY;
    let pressure = density_error * PRESSURE_MULTIPLIER;
    return pressure;
}

pub fn predict_positions(particle_data: &mut ParticleData){
    for index in 0..particle_data.positions.len() {
        particle_data.predicted_positions[index][0] = particle_data.positions[index][0] + particle_data.velocities[index][0];
        particle_data.predicted_positions[index][1] = particle_data.positions[index][1] + particle_data.velocities[index][1];
    }
}
