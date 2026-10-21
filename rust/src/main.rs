use piston_window::{Texture, TextureSettings, ResizeEvent, Window};
use image::{ImageBuffer, Rgba};
use simulation_project::*;

const PARTICLE_COUNT: usize = 300;
// Density radius as a fraction of window width for DPI-independent behavior
const DENSITY_RADIUS_FRACTION: f64 = 1.0 / 12.0;

fn main() {
    let initial_size = [1200.0, 800.0];

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Simulation", initial_size)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Use the actual logical window size reported by the OS (accounts for DPI)
    let piston_window::Size { width, height } = window.size();
    let mut window_size = [width, height];
    let mut density_radius = window_size[0] * DENSITY_RADIUS_FRACTION;

    let mut particles = ParticleData {
        mass: 1.0,
        positions: vec![[0.0, 0.0]; PARTICLE_COUNT],
        predicted_positions: vec![[0.0, 0.0]; PARTICLE_COUNT],
        velocities: vec![[0.0, 0.0]; PARTICLE_COUNT],
        densities: vec![0.0; PARTICLE_COUNT],
    };
    set_positions(&mut particles.positions, window_size, false);

    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(window_size[0] as u32, window_size[1] as u32);

    let mut texture_context = window.create_texture_context();
    let texture = Texture::from_image(
        &mut texture_context,
        &buffer,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {
        // Handle resize and DPI scale factor changes
        if let Some(args) = event.resize_args() {
            window_size = [args.window_size[0], args.window_size[1]];
            density_radius = window_size[0] * DENSITY_RADIUS_FRACTION;
        }

        simulation_step(&mut particles, density_radius, window_size);

        window.draw_2d(&event, |c, g, _device| {
            piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
            render_particles(&particles, window_size[0] * 0.004, c, g);
        });
    }
}

fn render_particles<G: piston_window::Graphics>(
    particle_data: &ParticleData,
    particle_size: f64,
    c: piston_window::Context,
    g: &mut G,
) {
    let size = particle_size;
    for position in &particle_data.positions {
        let x = position[0];
        let y = position[1];

        let rectangle = piston_window::rectangle::square(x - size, y - size, size * 2.0);
        piston_window::ellipse([1.0, 1.0, 1.0, 1.0], rectangle, c.transform, g);
    }
}
