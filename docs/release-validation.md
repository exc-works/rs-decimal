# Release candidate validation

The complete implementation currently covers 166 baseline APIs, including platform mappings, with 21,579 fixed differential cases. Version 0.1.0-alpha.1 has not yet been published. Early stacked PRs were merged into intermediate branches; PR #22 integrates the complete implementation into main. Check GitHub for its current merge status. Subsequent changes must pass CI again at their latest commits.

Historical validation on 2026-11-06, macOS arm64, Rust 1.99.1, Go 1.27.3: 512 fixed random cases, outputs/states for all 45 must-succeed cases, and independent certificates passed. A 20-second randomized run passed 251,648 cases and 188,624 independent arithmetic certificates (seed=14598561). This evidence applies to the commit tested then and does not replace checks for later commits.

CI covers MSRV 1.85/stable, all features, individual features, debug/release, packaged artifacts, paired Linux 32/64-bit Go/Rust, macOS/Windows, and live PostgreSQL. Weekly/manual randomized jobs save seeds, failing requests, and both implementations' responses. The cron schedule takes effect only after the workflow reaches the default branch.

Protocol failures preserve complete receiver state. Tests cover binary/BSON interoperability in both directions, PostgreSQL's maximum legal NUMERIC values/NULL/arrays, raw configuration tokens, and exact validation rules. Gates check the 11 frozen upstream files and the inventory of 166 declarations.

## Repeatable performance baseline

The medians below come from three batch calls per group using Go/Rust release, with complete results checked on every run. They include subprocess startup and JSONL overhead, are not pure algorithm microbenchmarks, and do not imply that Rust is generally faster. The representation/parsing group includes long numeric text, where Rust is currently substantially slower. These measurements establish a baseline for future optimization without changing numerical semantics for speed.

| Input group | Count | Go median, ms | Rust median, ms |
| --- | ---: | ---: | ---: |
| representation | 128 | 5.226 | 25.639 |
| arithmetic | 128 | 3.195 | 2.428 |
| conversion | 128 | 2.990 | 2.254 |
| roots | 111 | 2.931 | 2.245 |
| transcendentals | 128 | 3.567 | 3.996 |
| formatting | 128 | 3.044 | 2.157 |
| json | 128 | 3.283 | 2.599 |
| binary | 124 | 2.929 | 2.381 |
| yaml | 128 | 3.683 | 2.661 |
| xml | 128 | 3.359 | 2.452 |
| bson | 128 | 3.907 | 3.938 |
| sql | 128 | 3.166 | 2.628 |

Rerun with `python3 tools/conformance/benchmark.py --go .cache/go-reference --rust target/release/decimal-conformance`. CI saves core-gates, fuzz, and benchmark JSON as artifacts. Timings have no absolute threshold across machines; correctness failures always block acceptance.

See the [release process](releasing.md) for publication steps and independent consumer validation. Compatibility differences are collected in the [API matrix](api-compatibility.md).
