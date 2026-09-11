# Hyperfine Benchmark Suite

Comparative benchmarks between the `bump` Rust CLI and `bump-my-version` Python CLI.

## Results

### CLI Cold-Start Comparison

Measured on x86-64 Linux, release build with `RUSTFLAGS="-C target-cpu=native"`.

| Tool                       | Mean       | Min      | Max      | Speedup |
| -------------------------- | ---------- | -------- | -------- | ------- |
| **`bump` (Rust)**          | **2.4 ms** | 1.8 ms   | 3.2 ms   | **1×**  |
| `bump-my-version` (Python) | 482.3 ms   | 473.8 ms | 488.0 ms | 0.005×  |

**CLI speedup: ~200× faster.**

```
Benchmark 1: bump --config-file .bumpversion.toml --bump patch --dry-run
  Time (mean ± σ):       2.4 ms ±   0.5 ms    [User: 0.8 ms, System: 1.6 ms]
  Range (min … max):     1.8 ms …   3.2 ms    10 runs

Benchmark 2: bump-my-version bump patch --dry-run
  Time (mean ± σ):     482.3 ms ±   7.5 ms    [User: 430.1 ms, System: 52.8 ms]
  Range (min … max):   473.8 ms … 488.0 ms     3 runs
```

## Running the Full Suite

```sh
chmod +x run_comparison.sh
./run_comparison.sh
```

The script:

1. Builds the release binary if not present
2. Benchmarks scenario 1: dry-run (best case)
3. Benchmarks scenario 2: 10-file config
4. Compares against `bump-my-version` if installed
5. Exports JSON + Markdown results to `results/`

## Scenarios

| Scenario    | Description                                         |
| ----------- | --------------------------------------------------- |
| Best case   | `--dry-run`, single file, version in middle of file |
| Worst case  | Version string at very end of 100K-line file        |
| Multi-file  | 10 registered files in config                       |
| Pre-release | Cyclic stage bump (alpha → beta → rc → stable)      |

## Methodology

- Warmup: 3 runs discarded before measurement
- Measured runs: 10
- Shell startup excluded with `-N` (`--shell=none`)
- System load: quiet system, no background compilation
- Binary: stripped, statically linked, `target-cpu=native`
