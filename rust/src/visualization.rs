//this file has a bunch of function and whatnot for testing purposes
use std::time::Instant;
use crate::types::*;
use crate::sph::*;
use image::{ImageBuffer, Rgba};

#[allow(dead_code)]
pub fn visualize_density<G: piston_window::Graphics>(
    position: [f64; 2],
    radius: f64,
    c: piston_window::Context,
    g: &mut G,
) {
    draw_circle_border(
        [position[0], position[1]],
        radius,
        2.0,
        [1.0, 0.0, 0.0, 1.0],
        c,
        g,
    );
}

#[allow(dead_code)]
pub fn draw_circle_border<G: piston_window::Graphics>(
    center: [f64; 2],
    radius: f64,
    width: f64,
    color: [f32; 4],
    c: piston_window::Context,
    g: &mut G,
) {
    // Draw circle border by drawing line segments around the circumference
    let segments = 64;
    let angle_step = std::f64::consts::PI * 2.0 / segments as f64;

    for i in 0..segments {
        let angle1 = i as f64 * angle_step;
        let angle2 = ((i + 1) % segments) as f64 * angle_step;

        let x1 = center[0] + radius * angle1.cos();
        let y1 = center[1] + radius * angle1.sin();
        let x2 = center[0] + radius * angle2.cos();
        let y2 = center[1] + radius * angle2.sin();

        piston_window::line(color, width, [x1, y1, x2, y2], c.transform, g);
    }
}

#[allow(dead_code)]
pub fn draw_function(buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let (w, h) = buffer.dimensions();

    // white background
    for p in buffer.pixels_mut() {
        *p = Rgba([255, 255, 255, 255]);
    }

    // y = sin(x)
    for x in 0..w {
        for y in 0..h {
            let shade = (example_function(x as f64, y as f64) * 255.0) as u8;
            buffer.put_pixel(x, y, Rgba([shade, shade, shade, 255]));
        }
    }
}

pub fn draw_approximation(buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, particle_data: &ParticleData, radius: f64) {
    println!("Drawing approximation with {} positions and radius {}", particle_data.positions.len(), radius);
    let start = Instant::now();
    let (w, h) = buffer.dimensions();

    // white background
    for p in buffer.pixels_mut() {
        *p = Rgba([255, 255, 255, 255]);
    }

    // y = sin(x)
    for x in 0..w {
        for y in 0..h {
            let point = [x as f64, y as f64];
            let shade = (approximate_value(&point, &particle_data, radius) * 255.0) as u8;
            buffer.put_pixel(x, y, Rgba([shade, shade, shade, 255]));
        }
    }

    let duration = start.elapsed();
    println!("Approximation drawn in: {:?} ms", duration.as_millis());
}

#[allow(dead_code)]
pub fn render_particles_from_function(particle_data: &ParticleData, size: f64, c: piston_window::Context, g: &mut impl piston_window::Graphics) {
    for position in &particle_data.positions {
        let x = position[0];
        let y = position[1];
        let shade = example_function(x as f64, y as f64) as f32 ;
        let rectangle = piston_window::rectangle::square(x - size, y - size, size * 2.0);
        piston_window::ellipse([shade, shade, shade, 1.0], rectangle, c.transform, g);
    }
}
