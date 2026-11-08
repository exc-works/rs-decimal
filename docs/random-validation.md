# Randomized validation and failure replay

Build decimal-conformance in release mode, along with tools/go-reference and tools/go-oracle, then run:

```sh
python3 tools/conformance/fuzz.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle --seconds 60 --seed 14598561
python3 tools/conformance/fuzz.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle --replay .cache/fuzz-failure.json
```

Each batch covers basic arithmetic, odd roots, rescale, ln/log2/log10/exp, and parsing. Protocol cases randomly select successful and known malformed JSON/YAML/XML/Go binary/BSON inputs. Logarithm inputs include values close to 1; exponential inputs are restricted to [-20, 20] to avoid treating arbitrary resource exhaustion as expected success. Operation types and expected outcomes are determined before calling either implementation; a shared failure cannot substitute for success. Every successful numerical result is also certified by the independent rational/interval oracle. Protocol checks cover error categories, coefficient, scale, initialization, and nullable validity; failures must preserve receiver state.

The report at `.cache/fuzz-report.json` records coverage counts by operation/protocol/outcome. Failure artifacts save complete requests and expected outcomes for exact --replay, independent of future generator versions. `--seconds` is a minimum run duration checked between batches. Each subprocess has a `--timeout` (60 seconds by default), and the CI job has an overall timeout. Currently, if a timed-out process has no complete JSONL output, its response list is empty; this cannot establish which prefix it processed.

Unit tests for PostgreSQL's private wire codec cover 512 deterministically generated values, every frame truncation, special signs, out-of-range dscale, trailing bytes, out-of-range base-10000 digits, and implicit precision loss. Live-server tests additionally cover the maximum legal NUMERIC value, NULL/arrays, and text/binary paths. Resource-boundary tests check rejection of excessive precision and preservation of state on failure; they do not promise constant resource costs for arbitrary-length inputs.

Local extended validation on 2026-11-07: seed=14598561, 21,112 cases in 15 seconds, including 16,240 independently certified numerical results. Successes/failures were covered for all ten numerical operations and five protocols. Subsequent commits must use their latest run reports as evidence.
