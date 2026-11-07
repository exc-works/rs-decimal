# Core preview validation

The scope is PRs 01–09 and 10a. Protocol and ecosystem adapters remain in progress and are not claimed complete at this checkpoint. The crate is unpublished; crates.io name availability must still be checked before publication.

On 2026-11-06, local macOS arm64, Rust 1.99.1, and Go 1.27.3 passed 18,041 fixed differential cases, 512 random cases with a fixed seed and their independent certificates, and all 45 must-succeed cases with exact matches and independent certification. Debug/release unit regressions, fmt, Clippy, baseline integrity, cargo package, and standalone tests of the package passed. The package includes source, Rust tests, examples, and licenses, but excludes Go tools and fixtures. MSRV 1.85 is tested in CI.

The paired 32-bit gate natively executes i686 Rust and GOARCH=386 Go in Linux CI, including debug/release unit tests, the full differential suite, random certificates, and all 45 fixed success certificates. Linux 64-bit runs the same gates. Both were actually executed and passed at commit 009f2cf in PR #10, with core-gates JSON artifacts uploaded. No local 32-bit execution result is claimed.

Performance samples compare every output before recording timings: 512 mixed arithmetic/root requests, including subprocess startup and JSONL encoding/decoding. Medians of three runs were 5.865 ms for Go and 4.720 ms for Rust release. This sample establishes only a repeatable batch baseline, not an individual algorithm speedup or a cross-machine performance guarantee. Rerun with:

```sh
python3 tools/conformance/core-gates.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle
```

The randomized smoke-test seed is 0xDEC1A1; on failure, complete requests/results are written to .cache/core-fuzz-failure.json. Known review regressions include zero divisors at extreme scale, complete error-state preservation, oracle encoding mutations, and root/logarithm rounding boundaries; all are part of regular tests. Extended fuzz jobs are added in PR 14.
