# Ownership API workload measurements

Measured 2026-11-07 on macOS 27.2 arm64, rustc 1.99.1, at `69637925f47e2bc1b2df05b96fe6fc49b2d0c666`. Run `cargo bench --locked --bench core`; see [raw measurements](performance-owned-api.csv). The harness uses 100 ms warmup and seven approximately 50 ms samples with bounded iteration counts. Builds/validation completed before timing.

This compares two API paths within the same implementation, not a new baseline revision. Both pipelines begin by cloning the same prepared Decimal, then add and subtract the same borrowed operand and rescale to the unchanged scale. Both therefore include identical owned-input acquisition costs and result destruction. The consuming path can reuse intermediate coefficients. Suffixes 6/1000 indicate coefficient decimal digits.

| Workload | Median ns/op | Min | Max |
| --- | ---: | ---: | ---: |
| pipeline_borrowed_6 | 38.49 | 38.23 | 38.68 |
| pipeline_owned_6 | 21.56 | 21.50 | 21.71 |
| coefficient_copy_6 | 7.13 | 7.12 | 7.14 |
| coefficient_borrow_6 | 0.84 | 0.82 | 0.84 |
| coefficient_clone_then_move_6 | 6.94 | 6.92 | 7.01 |
| pipeline_borrowed_1000 | 76.86 | 76.07 | 77.31 |
| pipeline_owned_1000 | 33.64 | 33.56 | 33.89 |
| coefficient_copy_1000 | 14.82 | 14.75 | 14.90 |
| coefficient_borrow_1000 | 0.84 | 0.71 | 0.85 |
| coefficient_clone_then_move_1000 | 16.82 | 16.65 | 17.04 |

The 6-digit consuming pipeline took 44.0% less time for this workload.

The 1000-digit consuming pipeline took 56.2% less time for this workload.

Coefficient borrowing returns a reference and performs less work than producing an independent copy; it is a different ownership contract, not a drop-in speedup for callers that need ownership. clone_then_move includes the clone needed to supply a fresh input on every iteration: moving out the coefficient does not erase upstream cloning costs. No universal allocation-free or speed guarantee is implied.

Validation: workspace all-feature debug/release and rustdoc tests, all-target/all-feature Clippy with warnings denied, full-state consuming/borrowed equivalence tests, compile-fail ownership examples, 21,579 fixed differential cases, 512 seeded random/oracle cases, and 2,853 arithmetic plus 437 rounding oracle certificates. New APIs are validated directly because the pinned Go inventory does not contain them.
