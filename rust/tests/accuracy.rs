mod support;

use simulation_project as current;
use support::*;

#[test]
fn kernels_match_original_at_support_edges() {
    for radius in [0.1, 10.0, 100.0, 250.0] {
        for distance in [
            0.0,
            radius * 0.25,
            radius * (1.0 - 1e-12),
            radius,
            radius * (1.0 + 1e-12),
        ] {
            assert_close(
                current::smoothing_kernel(radius, distance),
                original::smoothing_kernel(radius, distance),
                "kernel",
            );
            assert_close(
                current::smoothing_kernel_derivative(radius, distance),
                original::smoothing_kernel_derivative(radius, distance),
                "gradient",
            );
        }
    }
}

#[test]
fn density_and_pressure_match_original() {
    let mut cases = vec![
        fixture(1, false),
        fixture(37, false),
        fixture(64, true),
        particles(vec![
            [200.0, 200.0],
            [200.0, 200.0],
            [300.0, 200.0],
            [1000.0, 700.0],
        ]),
    ];
    for data in &mut cases {
        data.mass = 1.7;
        // Density and pressure intentionally read different position arrays in the original.
        current::predict_positions(data);
        for radius in [10.0, RADIUS, 250.0] {
            for i in 0..data.positions.len() {
                assert_close(
                    current::calculate_density(data, i, radius),
                    original::calculate_density(data, i, radius),
                    "density",
                );
            }
            current::precalculate_densities(data, radius);
            for zero_density in [false, true] {
                if zero_density {
                    data.densities[0] = 0.0;
                }
                for i in 0..data.positions.len() {
                    assert_values(
                        &current::calculate_pressure_force(data, i, radius),
                        &original::calculate_pressure_force(data, i, radius),
                        "pressure force",
                    );
                }
            }
        }
    }
}

#[test]
fn trajectories_match_original_at_every_step() {
    let mut walls = particles(vec![
        [0.0, 100.0],
        [1200.0, 100.0],
        [100.0, 0.0],
        [100.0, 800.0],
        [-0.1, 400.0],
        [1200.1, 400.0],
    ]);
    walls.velocities.fill([0.2, -0.3]);
    let mut non_unit_mass = fixture(37, false);
    non_unit_mass.mass = 1.7;
    let cases = vec![
        particles(vec![]),
        fixture(1, false),
        fixture(37, false),
        fixture(128, false),
        fixture(64, true),
        non_unit_mass,
        walls,
        particles(vec![[500.0, 400.0], [500.0, 400.0], [1000.0, 700.0]]),
    ];
    for (case, mut actual) in cases.into_iter().enumerate() {
        let mut expected = copy(&actual);
        for step in 1..=100 {
            current::simulation_step(&mut actual, RADIUS, WINDOW);
            original::simulation_step(&mut expected, RADIUS, WINDOW);
            assert_state(&actual, &expected, &format!("case {case}/step {step}"));
        }
        assert_state(&actual, &expected, &format!("case {case}/final"));
    }
}

#[test]
fn error_check_rejects_drift_and_nonfinite_results() {
    assert_close(1.0 + 1e-10, 1.0, "roundoff");
    for (actual, expected) in [
        (1.01, 1.0),
        (f64::NAN, f64::NAN),
        (f64::INFINITY, f64::INFINITY),
        (1.0, f64::NAN),
    ] {
        assert!(
            std::panic::catch_unwind(|| assert_close(actual, expected, "intentional failure"))
                .is_err()
        );
    }
}

#[test]
fn error_metrics_report_known_values() {
    let (max, rmse) = assert_values(&[1e-10, -1e-10], &[0.0, 0.0], "metrics");
    assert!((max - 1e-10).abs() < 1e-20);
    assert!((rmse - 1e-10).abs() < 1e-20);
}
