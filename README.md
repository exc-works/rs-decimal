# rs-decimal

rs-decimal is a Rust port that preserves the numerical and state contract of exc-works/decimal. It implements the numerical core, formatting, Text/Param, JSON/Serde, Go binary, YAML/XML/BSON, Scan/Value, PostgreSQL SQLx, exact validation, and configuration decoding. All 166 baseline APIs have an implementation or an explicit platform mapping; the crate has not yet been published.

## Development

The MSRV is Rust 1.85 (edition 2024); CI checks both MSRV and stable. Core dependencies are pinned versions of num-bigint, num-integer, num-traits, num-rational, and ryu. There are currently no default features.

```sh
cargo check --locked
cargo test --locked --workspace --all-features
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
python3 tools/check-baseline.py
```

The [API compatibility matrix](docs/api-compatibility.md) records the implementation and validation status of each entry point. Original contracts and fixtures are in `testdata/upstream/`; see `LICENSE` and `NOTICE` for licensing and provenance.

## Implementation documents

- [Implementation plan](docs/implementation-plan.md): goals, scope, milestones, risks, and completion criteria.
- [Technical design](docs/technical-design.md): types, algorithms, errors, conversions, codecs, and validation protocols.
- [PR plan](docs/pr-plan.md): dependency order, deliverables, and acceptance criteria for each PR.

The plan is based on Go repository commit `a8791208e2d15483cf652cd2814b74b0c1a84187` and its v0.8.0 numerical contract. The repository is named `rs-decimal`; publication under the same name on crates.io still needs to be verified.

For user-facing API documentation, run `cargo doc --all-features --open`. See the [release process](docs/releasing.md) for release checks and the [CHANGELOG](CHANGELOG.md) for version changes.

## Using the numerical core

```rust
use rs_decimal::{Decimal, GoFormat, RoundingMode};
let price = Decimal::must_from_string("12.350");
assert_eq!(price.string_with_trailing_zeros(), "12.350");
let total = price.mul_exact(&Decimal::new(3));
assert_eq!(total.string(), "37.05");
let third = Decimal::new(1).quo_with_prec(&Decimal::new(3), 2, RoundingMode::HalfEven);
assert_eq!(third.string_with_trailing_zeros(), "0.33");
assert_eq!(Decimal::new(2).sqrt_with_prec(5)?.string(), "1.41421");
assert_eq!(price.format_go(GoFormat { verb: 'f', precision: Some(2), ..GoFormat::default() }), "12.35");
# Ok::<(), rs_decimal::DecimalError>(())
```

Decimal preserves its coefficient and scale. Default is uninitialized zero; numerical equality does not imply identical representation. Arithmetic leaves inputs unchanged. Ln, roots, and similar operations return Result; the default Log2 entry point and contracts such as negative precision or division by zero use typed DecimalError panics. Among the seven rounding modes, Up rounds away from zero and Ceiling rounds toward positive infinity.

Version `0.1.0-alpha.1` has not yet been published; see the API matrix for preview scope, error rules, and validation evidence. `tools/conformance/core-gates.py` runs the full differential suite, 512 random cases with a fixed seed, and an independent oracle, then checks every result and reports batch timings including process/JSONL overhead. These timings do not establish arithmetic speedups. CI executes paired i686 Go/Rust and 64-bit tests in debug/release and saves timing reports. Extreme native-scale fixtures use isize::MAX in the 32-bit pairing; i64/u64 value ranges remain unchanged.

## Optional JSON and Serde

`json` provides marshal_json/unmarshal_json, preserving JSON number tokens directly for high-precision external input. For plain Decimal, null leaves the existing value unchanged; for NullDecimal, null clears the state. `serde` provides Serialize/Deserialize using exact strings, integers, and null. Deserializing null into a new object produces the default value, with no previous state. Generic Deserialize rejects floating-point input; when reading decimal tokens through serde_json, use the json entry points or encode decimal values as strings. The two features can be enabled independently.

`yaml` uses event parsing that preserves source text to handle scalar documents and tags. marshal_yaml returns a string or optional string for a YAML emitter. Go binary interoperability uses marshal_binary/unmarshal_binary, preserving the wire format and failure atomicity.

`xml` provides decoding from raw element text and decoded attribute values. Encoding returns character content or an attribute value; the caller's XML emitter handles names and escaping. Empty elements clear the value, and failures preserve the previous state.

`bson` provides BSON type-byte/raw-byte adapters for string, double, int32, int64, Decimal128, and null, without a MongoDB client dependency.

`ScanValue` and `scan/value` provide driver-neutral SQL input/output semantics. Text preserves precision, SQL NULL uses NullDecimal's valid state, and floating-point NaN/Infinity are rejected.

See [release candidate validation](docs/release-validation.md) for complete protocol gates, grouped timings, and extended randomized validation. Run `python3 tools/check-release-matrix.py` to check the status of promised APIs; repository workflows do not publish the crate automatically.

`postgres` provides exact NUMERIC/NUMERIC[] implementations for SQLx 0.8.6, with nullable support and out-of-range rejection. The application selects its runtime/TLS features. See [PostgreSQL usage and live validation](docs/postgres.md).

`validation` provides 12 exact numerical rules through `ValidationRule`, scale checks, and 13 sets of language templates through `ValidationMessages`. Invalid rules return errors at construction; there is no global registration or floating-point comparison.


## Exact configuration fields

Enable `config` to use Serde field adapters specifically for raw JSON input. Plain Decimal configuration fields reject null/empty strings; NullDecimal accepts them. Struct attributes determine whether missing fields use defaults. The typed `ConfigValue` also provides the value-conversion rules of Go DecodeHook.

```rust
#[derive(serde::Deserialize)]
struct Settings {
    #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]
    amount: rs_decimal::Decimal,
    #[serde(default, deserialize_with = "rs_decimal::config::deserialize_json_nullable")]
    limit: rs_decimal::NullDecimal,
}
let settings: Settings = serde_json::from_str(
    r#"{"amount":9007199254740993.00000000000000001}"#
)?;
# Ok::<(), serde_json::Error>(())
```

Optional features are serde, json, yaml, xml, bson, postgres, validation, and config; there are no default features. The generic Serde Decimal visitor rejects floating-point values, while the config JSON field adapters above use RawValue to preserve number tokens. Their purposes and null rules differ; see the API matrix.

Run the complete configuration example with `cargo run --example configuration --features config,validation`.

For maintenance, see [module responsibilities and numerical invariants](docs/maintaining.md). Run in-process benchmarks with `cargo bench --bench core`; see [randomized validation](docs/random-validation.md) for fuzzing and replay.
