# 2D Fluid Simulation
A simple 2-dimensional fluid simulation written in rust

## Optimization checks

Run the headless regression suite from the repository root:

```sh
RAYON_NUM_THREADS=4 cargo test --locked --no-default-features --test accuracy
```

Add `-- --nocapture` to see final maximum absolute error and RMSE for positions,
predicted positions, velocities, and density. Tests compare kernels, density,
pressure forces, and every step of 100-step rollouts against the frozen numerical
code from commit `ebc37f67c2fd971dd615778728e9d52d3365c2a5` in
`rust/tests/support/original/`. That copy only shares the production state type;
it must not call production numerical helpers or be updated alongside optimizations.
Empty, isolated, coincident, clustered, moving, wall/outside-wall, non-square-count,
zero-neighbor-density, non-unit-mass, and kernel-cutoff cases are included.

Each scalar must be finite and satisfy
`abs(current - original) <= 1e-9 + 1e-8 * abs(original)`.
This is a strict behavior-preservation gate for conventional optimizations.
Intentional changes to physics or approximate ML corrections need separately
justified tolerances and physical benchmarks; passing this suite does not prove
the original solver physically accurate.

Run the release performance benchmark with the same thread count:

```sh
RAYON_NUM_THREADS=4 cargo bench --locked --no-default-features --bench simulation
```

The benchmark checks accuracy first, warms both implementations, alternates their
measurement order, and uses identical deterministic states at 64, 300, and 1024
particles in spread and clustered layouts. Each row reports nine samples of five
steps: median/min/max time per step, particle updates per second, allocation calls
per step, total requested allocation bytes per step, and the original/current
median runtime ratio (above 1 means the current code is faster).

Input construction is outside timing and allocation measurement. Allocations are
counted in a separate untimed run across all Rayon threads; reallocation counts
as one call and its full requested size. Requested bytes are allocation traffic,
not peak live memory or process RSS. Timing includes a disabled allocator-counter
check, so treat very small timing differences cautiously. Thread-pool startup and
rendering are excluded. No timing threshold fails a test: repeat on the same idle
machine, toolchain, and thread count to judge performance regressions. No speedup
is expected until the production implementation changes.
