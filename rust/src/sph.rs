use std::f64::consts::PI;
use rayon::prelude::*;

use crate::types::*;

pub fn smoothing_kernel(radius: f64, distance: f64) -> f64 {
    if distance >= radius { return 0.0; }
    //normalized is so that there arent wildly large values messing up the density calculation
    let normalized_radius = radius / 100.0;
    let normalized_distance = distance / 100.0;
    let volume = PI * normalized_radius.powf(4.0) / 6.0;
    let value = normalized_radius - normalized_distance;
    return value * value / volume;
}

pub fn smoothing_kernel_derivative(radius: f64, distance: f64) -> f64 {
    if distance >= radius { return 0.0; }
    let normalized_radius = radius / 100.0;
    let normalized_distance = distance / 100.0;
    let scale = 12.0 / (normalized_radius.powf(4.0) * PI);
    return (normalized_distance - normalized_radius) * scale;
}

pub fn calculate_density(particle_data: &ParticleData, index: usize, smoothing_radius: f64) -> f64 {
    let mut density: f64 = 0.0;
    let reference_position = particle_data.predicted_positions[index];

    for j in 0..particle_data.predicted_positions.len() {
        let other_position = particle_data.predicted_positions[j];
        let dx = reference_position[0] - other_position[0];
        let dy = reference_position[1] - other_position[1];
        let distance = (dx * dx + dy * dy).sqrt();
        let influence = smoothing_kernel(smoothing_radius, distance);
        density += particle_data.mass * influence;
    }
    return density;
}

pub fn precalculate_densities(particle_data: &mut ParticleData, smoothing_radius: f64) {
    let densities: Vec<f64> = (0..particle_data.densities.len())
        .into_par_iter()
        .map(|i| calculate_density(&particle_data, i, smoothing_radius))
        .collect();

    particle_data.densities = densities;
}

pub fn approximate_value(point: &[f64; 2], particle_data: &ParticleData, smoothing_radius: f64) -> f64 {
    let mut value = 0.0;
    let mass = particle_data.mass;
    let radius_sq = smoothing_radius * smoothing_radius;
    let positions = &particle_data.positions;
    let densities = &particle_data.densities;

    for i in 0..positions.len() {
        let dx = point[0] - positions[i][0];
        let dy = point[1] - positions[i][1];
        let distance_sq = dx * dx + dy * dy;

        if distance_sq > radius_sq {
            continue;
        }

        let distance = distance_sq.sqrt();
        let influence = smoothing_kernel(smoothing_radius, distance);
        value += example_function(positions[i][0], positions[i][1]) * influence * mass / densities[i];
    }

    return value;
}

#[allow(dead_code)]
pub fn example_function(x: f64, y: f64) -> f64 {
    return (((y / 50.0) + (x / 50.0).sin()).cos() + 1.0) / 2.0;
}
