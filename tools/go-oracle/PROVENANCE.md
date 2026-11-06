# Oracle source provenance

Source: `https://github.com/exc-works/decimal`, revision `a8791208e2d15483cf652cd2814b74b0c1a84187`. MIT license: [LICENSE](../../LICENSE).

- `reference_rounding.go`: `roundedRationalReference` from `arithmetic_scale_test.go` (whole original file SHA-256 `c13c2a3699270e8b13a1f814153a4a22a495551d36149a452e9656b5911c6095`). Changes: package name, standalone mode constants and `RoundingMode` parameter changed to `int`; exact-neighbor algorithm unchanged.
- Request handling and exact-addition verification are new test infrastructure. No production decimal package is imported.
