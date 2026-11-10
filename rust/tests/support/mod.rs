#![allow(dead_code)]
pub mod original;

use simulation_project::ParticleData;

pub const WINDOW: [f64; 2] = [1200.0, 800.0];
pub const RADIUS: f64 = 100.0;
pub const ABS_TOL: f64 = 1e-9;
pub const REL_TOL: f64 = 1e-8;

pub fn particles(positions: Vec<[f64; 2]>) -> ParticleData {
    let count = positions.len();
    ParticleData {
        mass: 1.0,
        predicted_positions: positions.clone(),
        positions,
        velocities: vec![[0.0; 2]; count],
        densities: vec![0.0; count],
    }
}

pub fn fixture(count: usize, clustered: bool) -> ParticleData {
    let columns = (count as f64).sqrt().ceil() as usize;
    let spacing = if clustered { 2.0 } else { 22.0 };
    let mut data = particles(
        (0..count)
            .map(|i| {
                [
                    100.0 + (i % columns) as f64 * spacing,
                    100.0 + (i / columns) as f64 * spacing,
                ]
            })
            .collect(),
    );
    for (i, velocity) in data.velocities.iter_mut().enumerate() {
        *velocity = [(i % 7) as f64 * 0.03 - 0.09, (i % 11) as f64 * 0.02 - 0.1];
    }
    data
}

pub fn copy(data: &ParticleData) -> ParticleData {
    ParticleData {
        mass: data.mass,
        positions: data.positions.clone(),
        predicted_positions: data.predicted_positions.clone(),
        velocities: data.velocities.clone(),
        densities: data.densities.clone(),
    }
}

pub fn assert_close(actual: f64, expected: f64, context: &str) {
    assert!(
        actual.is_finite() && expected.is_finite(),
        "{context}: nonfinite {actual} / {expected}"
    );
    let error = (actual - expected).abs();
    assert!(
        error <= ABS_TOL + REL_TOL * expected.abs(),
        "{context}: actual={actual:e}, original={expected:e}, abs_error={error:e}"
    );
}

// Return max absolute error and RMSE as useful diagnostics for approximate optimizations.
pub fn assert_values(actual: &[f64], expected: &[f64], context: &str) -> (f64, f64) {
    assert_eq!(actual.len(), expected.len(), "{context}: length");
    let mut max_error: f64 = 0.0;
    let mut squared_error = 0.0;
    for (i, (&a, &b)) in actual.iter().zip(expected).enumerate() {
        assert_close(a, b, &format!("{context}[{i}]"));
        max_error = max_error.max((a - b).abs());
        squared_error += (a - b).powi(2);
    }
    (
        max_error,
        (squared_error / actual.len().max(1) as f64).sqrt(),
    )
}

pub fn assert_state(actual: &ParticleData, expected: &ParticleData, context: &str) {
    assert_close(actual.mass, expected.mass, context);
    for (name, a, b) in [
        ("positions", &actual.positions, &expected.positions),
        (
            "predicted_positions",
            &actual.predicted_positions,
            &expected.predicted_positions,
        ),
        ("velocities", &actual.velocities, &expected.velocities),
    ] {
        let metrics = assert_values(
            a.as_flattened(),
            b.as_flattened(),
            &format!("{context}/{name}"),
        );
        if context.ends_with("final") {
            println!(
                "{context}/{name}: max_abs={:e}, rmse={:e}",
                metrics.0, metrics.1
            );
        }
    }
    let metrics = assert_values(
        &actual.densities,
        &expected.densities,
        &format!("{context}/density"),
    );
    if context.ends_with("final") {
        println!(
            "{context}/density: max_abs={:e}, rmse={:e}",
            metrics.0, metrics.1
        );
    }
}
