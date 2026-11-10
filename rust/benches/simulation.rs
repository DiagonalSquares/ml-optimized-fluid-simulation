#[path = "../tests/support/mod.rs"]
mod support;

use simulation_project::{ParticleData, simulation_step};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::time::Instant;
use support::*;

struct CountingAllocator;
static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);

fn record_allocation(bytes: usize) {
    if COUNTING.load(Relaxed) {
        ALLOCATIONS.fetch_add(1, Relaxed);
        ALLOCATED_BYTES.fetch_add(bytes, Relaxed);
    }
}

// Forward all operations unchanged; counters include Rayon worker allocations.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let result = unsafe { System.realloc(pointer, layout, size) };
        if !result.is_null() {
            record_allocation(size);
        }
        result
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
const STEPS: usize = 5;
const SAMPLES: usize = 9;
type Step = fn(&mut ParticleData, f64, [f64; 2]);

fn run(step: Step, data: &mut ParticleData) {
    for _ in 0..STEPS {
        step(black_box(data), black_box(RADIUS), black_box(WINDOW));
    }
    black_box(&data.positions);
    black_box(&data.velocities);
    black_box(&data.densities);
}

fn measure(step: Step, seed: &ParticleData) -> f64 {
    let mut data = copy(seed);
    let start = Instant::now();
    run(step, &mut data);
    start.elapsed().as_secs_f64() / STEPS as f64
}

fn allocations(step: Step, seed: &ParticleData) -> (usize, usize) {
    let mut data = copy(seed);
    ALLOCATIONS.store(0, Relaxed);
    ALLOCATED_BYTES.store(0, Relaxed);
    COUNTING.store(true, Relaxed);
    run(step, &mut data);
    COUNTING.store(false, Relaxed);
    (ALLOCATIONS.load(Relaxed), ALLOCATED_BYTES.load(Relaxed))
}

fn check_allocator() {
    ALLOCATIONS.store(0, Relaxed);
    ALLOCATED_BYTES.store(0, Relaxed);
    COUNTING.store(true, Relaxed);
    let data = black_box(vec![0_u8; 128]);
    COUNTING.store(false, Relaxed);
    assert!(ALLOCATIONS.load(Relaxed) >= 1);
    assert!(ALLOCATED_BYTES.load(Relaxed) >= 128);
    drop(data);
}

fn benchmark(count: usize, clustered: bool) {
    let seed = fixture(count, clustered);
    let mut actual = copy(&seed);
    let mut expected = copy(&seed);
    // Warm both implementations and reject inaccurate candidates before reporting speed.
    for step in 0..STEPS {
        simulation_step(&mut actual, RADIUS, WINDOW);
        original::simulation_step(&mut expected, RADIUS, WINDOW);
        assert_state(&actual, &expected, &format!("benchmark warmup step {step}"));
    }
    let mut current_times = [0.0; SAMPLES];
    let mut original_times = [0.0; SAMPLES];
    for sample in 0..SAMPLES {
        if sample % 2 == 0 {
            current_times[sample] = measure(simulation_step, &seed);
            original_times[sample] = measure(original::simulation_step, &seed);
        } else {
            original_times[sample] = measure(original::simulation_step, &seed);
            current_times[sample] = measure(simulation_step, &seed);
        }
    }
    current_times.sort_by(f64::total_cmp);
    original_times.sort_by(f64::total_cmp);
    let speedup = original_times[SAMPLES / 2] / current_times[SAMPLES / 2];
    for (name, times, step) in [
        (
            "original",
            original_times,
            original::simulation_step as Step,
        ),
        ("current", current_times, simulation_step as Step),
    ] {
        let (allocs, bytes) = allocations(step, &seed);
        let median = times[SAMPLES / 2];
        println!(
            "{count},{},{name},{:.3},{:.3},{:.3},{:.0},{:.1},{:.1},{speedup:.3}",
            if clustered { "clustered" } else { "spread" },
            median * 1e6,
            times[0] * 1e6,
            times[SAMPLES - 1] * 1e6,
            count as f64 / median,
            allocs as f64 / STEPS as f64,
            bytes as f64 / STEPS as f64
        );
    }
}

fn main() {
    check_allocator();
    eprintln!(
        "threads={}, samples={SAMPLES}, steps/sample={STEPS}; release={}",
        rayon::current_num_threads(),
        !cfg!(debug_assertions)
    );
    assert!(
        !cfg!(debug_assertions),
        "run with cargo bench (release profile)"
    );
    println!(
        "particles,layout,implementation,median_us_per_step,min_us,max_us,particle_updates_per_second,allocations_per_step,requested_bytes_per_step,current_speedup"
    );
    for count in [64, 300, 1024] {
        for clustered in [false, true] {
            benchmark(count, clustered);
        }
    }
}
