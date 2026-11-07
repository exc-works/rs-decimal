# In-process performance measurement

`cargo bench --locked --bench core` runs release microbenchmarks without additional dependencies. Inputs are constructed and known results checked before timing. Measurements include result allocation/destruction, but exclude process startup, JSONL transport, and printing. Each group warms up for at least 100 ms, then calibrates to roughly 50 ms per sample (100 to 2,000,000 operations) and reports the median, minimum, and maximum of seven samples. CI saves microbench.csv for regression investigation only; it sets no absolute speed threshold across machines.

## Local before/after comparison, 2026-11-07

macOS arm64, Rust 1.99.1, running the same benchmark source sequentially on the same machine. The baseline is `a04cdd6`; the optimized version is the commit introducing this report. See the [measurement data](performance-baseline.json) for raw sample ranges and actual iteration counts. Short samples taken before the warm-up correction were strongly affected by CPU startup frequency and are excluded from the conclusions below.

| Operation | Before, ns/op | After, ns/op | Time reduction |
| --- | ---: | ---: | ---: |
| parse_money | 60.86 | 18.32 | 69.9% |
| parse_integer | 61.03 | 26.27 | 57.0% |
| parse_exponent | 71.78 | 22.99 | 68.0% |
| parse_large | 2252.41 | 2204.32 | 2.1% |
| add_same_scale | 32.56 | 11.54 | 64.6% |
| add_mixed_scale | 59.32 | 44.47 | 25.0% |
| multiply_exact | 13.58 | 12.85 | 5.4% |
| divide_rounded | 76.93 | 77.41 | -0.6% |
| format_money | 50.67 | 50.71 | -0.1% |
| format_large | 10520.21 | 10525.48 | -0.1% |

For these inputs, short-number parsing took about 57–70% less time, same-scale addition about 65% less, and mixed-scale addition about 25% less. Large-number parsing improved only slightly; formatting and division showed no clear change. Small fluctuations in unchanged paths are environment/layout noise and cannot be attributed to the optimization. These figures are neither an overall speedup across all inputs nor a comparison of pure arithmetic speed against Go.

The optimization accumulates coefficients of up to 18 digits directly in a bounded u64, avoiding temporary concatenation and generic big-integer parsing; longer integers borrow the original digit bytes directly. Alignment for addition/subtraction and quotient/remainder borrows coefficients that need no scaling, allocating only for nonzero coefficients whose scale must increase. Full syntax validation, arbitrary-precision fallback, scale/zero state, and rounding rules are preserved.

Correctness validation passed: 21,579 fixed differential cases, 512 fixed random cases, and debug/release tests with all features. A 60-second extended randomized run passed 91,312 cases and 70,240 independent numerical certificates. Parsing regressions cover the 17/18/19-digit boundaries, signs, decimal-point positions, and long zero strings. Low-level changes must still pass the independent oracle every time; benchmark results cannot replace correctness checks.

To reproduce the baseline, apply the same `benches/core.rs` and `[[bench]]` configuration to the baseline commit, use a separate CARGO_TARGET_DIR, and run the before/after versions sequentially. Avoid sampling while compilation or randomized tests compete for CPU time. The existing end-to-end Go/Rust JSONL benchmark remains available; the two measurements answer different questions.
