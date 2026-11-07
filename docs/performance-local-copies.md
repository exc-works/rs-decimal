# Local coefficient-copy optimization measurements

Measured 2026-11-07 on macOS 27.2 arm64, rustc 1.99.1, release optimization. Baseline `33475b706fa76d2e286cb8bc59f2509b0ea77c18`; implementation `7024634825f595a6f2439253b90bd7841c06660e`. Both use exactly the `benches/core.rs` from the implementation commit. Separate target directories; builds and correctness checks finished before sequential baseline and implementation runs.

The harness warms each group for 100 ms, calibrates approximately 50 ms per sample (bounded iteration count), and reports seven-sample median/min/max. Results include allocation/destruction and exclude input preparation. New suffixes 6/1000 denote coefficient decimal digits. See [raw measurements](performance-local-copies.csv) for iteration counts and ranges; the earlier performance baseline is unchanged.

| Operation | Before ns/op | After ns/op | Time reduction |
| --- | ---: | ---: | ---: |
| safe_add_6 | 19.21 | 12.58 | 34.5% |
| safe_sub_6 | 20.07 | 13.56 | 32.4% |
| mixed_add_left_6 | 44.40 | 38.73 | 12.8% |
| mixed_sub_left_6 | 45.35 | 39.57 | 12.7% |
| mixed_add_right_6 | 44.31 | 38.43 | 13.3% |
| mixed_sub_right_6 | 46.11 | 39.20 | 15.0% |
| divide_positive_6 | 52.13 | 46.00 | 11.8% |
| divide_zero_6 | 25.90 | 15.52 | 40.1% |
| divide_negative_6 | 75.79 | 70.79 | 6.6% |
| format_negative_6 | 51.87 | 45.83 | 11.6% |
| format_scientific_6 | 99.57 | 95.32 | 4.3% |
| safe_add_1000 | 38.76 | 25.08 | 35.3% |
| safe_sub_1000 | 40.58 | 25.17 | 38.0% |
| mixed_add_left_1000 | 140.92 | 123.08 | 12.7% |
| mixed_sub_left_1000 | 125.14 | 107.93 | 13.8% |
| mixed_add_right_1000 | 146.97 | 120.35 | 18.1% |
| mixed_sub_right_1000 | 125.40 | 104.89 | 16.4% |
| divide_positive_1000 | 429.31 | 418.16 | 2.6% |
| divide_zero_1000 | 352.71 | 330.46 | 6.3% |
| divide_negative_1000 | 385.31 | 370.59 | 3.8% |
| format_negative_1000 | 10414.28 | 10414.77 | -0.0% |
| format_scientific_1000 | 10396.43 | 10407.63 | -0.1% |
| parse_money | 18.23 | 18.45 | -1.2% |
| parse_integer | 25.39 | 25.93 | -2.1% |
| parse_exponent | 23.16 | 23.25 | -0.4% |
| parse_large | 2204.79 | 2207.74 | -0.1% |
| add_same_scale | 11.41 | 11.66 | -2.2% |
| add_mixed_scale | 44.87 | 38.60 | 14.0% |
| multiply_exact | 13.11 | 13.35 | -1.8% |
| divide_rounded | 78.16 | 72.24 | 7.6% |
| format_money | 51.06 | 44.87 | 12.1% |
| format_large | 10517.73 | 10499.84 | 0.2% |

Small safe arithmetic took about 32–35% less time and the zero-shift small division case about 40% less time in this run. Large mixed-scale examples improved, while large formatting was essentially unchanged: avoiding its temporary absolute-value copy does not remove decimal conversion costs. Unchanged-path fluctuations and small differences are not evidence of universal gains. These are one-machine observations, not allocation counts or cross-language speed comparisons.

Reproduce with `cargo bench --locked --bench core`; copy the exact same benchmark into a clean archive of the baseline and use a separate CARGO_TARGET_DIR. Run versions sequentially after compilation. Expected outputs are checked before each measured family.

Validation at the implementation commit: workspace/all-feature debug and release tests, rustdoc tests, all-target/all-feature Clippy with warnings denied, 21,579 fixed Go differential cases, 512 seeded random cases with independent oracle, 2,853 arithmetic and 437 rounding oracle certificates. PostgreSQL service test remains ignored locally; CI platform/MSRV/32-bit/service results are reported on the PR rather than assumed.
