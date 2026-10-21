use crate::types::ParticleData;

#[repr(C)]
pub struct Grid {
    pub cell_size: f64,
    pub column_count: usize,
    pub row_count: usize,
    pub particle_indexes: Vec<usize>,
    pub keys: Vec<usize>,
    pub start_indexes: Vec<usize>,
}

pub fn create_grid(window_size: [f64; 2], cell_size: f64, particles: &ParticleData) -> Grid {
    let column_count = (window_size[0] / cell_size).ceil() as usize;
    let row_count = (window_size[1] / cell_size).ceil() as usize;
    let particle_count = particles.positions.len();
    let total_cell_count = column_count * row_count;

    Grid {
        cell_size,
        column_count,
        row_count,
        particle_indexes: (0..particle_count).collect(),
        keys: vec![0; particle_count],
        start_indexes: vec![usize::MAX; total_cell_count],
    }
}

pub fn create_cell_key(grid: &mut Grid, particles: &ParticleData) {
    for particle_index in 0..particles.predicted_positions.len() {
        let position = particles.predicted_positions[particle_index];
        let cell_x = (position[0] / grid.cell_size as f64).min((grid.column_count - 1) as f64) as usize;
        let cell_y = (position[1] / grid.cell_size as f64).min((grid.row_count - 1) as f64) as usize;
        grid.keys[particle_index] = cell_y * grid.column_count + cell_x;
    }
}

pub fn sort_keys(grid: &mut Grid) {
    let mut particle_key_pairs: Vec<(usize, usize)> = grid.particle_indexes.iter()
        .zip(grid.keys.iter())
        .map(|(&particle_index, &cell_key)| (particle_index, cell_key))
        .collect();

    particle_key_pairs.sort_by_key(|&(_, cell_key)| cell_key);

    for (sorted_index, (particle_index, cell_key)) in particle_key_pairs.into_iter().enumerate() {
        grid.particle_indexes[sorted_index] = particle_index;
        grid.keys[sorted_index] = cell_key;
    }

    build_start_indexes(grid);
}

fn build_start_indexes(grid: &mut Grid) {
    grid.start_indexes.fill(usize::MAX);

    for sorted_index in 0..grid.keys.len() {
        let cell_key = grid.keys[sorted_index];
        let is_first_occurrence = sorted_index == 0 || grid.keys[sorted_index - 1] != cell_key;
        if is_first_occurrence {
            grid.start_indexes[cell_key] = sorted_index;
        }
    }
}