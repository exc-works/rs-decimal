# Oracle source provenance

Source: `https://github.com/exc-works/decimal`, revision `a8791208e2d15483cf652cd2814b74b0c1a84187`. MIT license: [LICENSE](../../LICENSE).

- `reference_rounding.go`: `roundedRationalReference` from `arithmetic_scale_test.go` (whole original file SHA-256 `c13c2a3699270e8b13a1f814153a4a22a495551d36149a452e9656b5911c6095`). Changes: package name, standalone mode constants and `RoundingMode` parameter changed to `int`; exact-neighbor algorithm unchanged.
- Request handling and exact-addition verification are new test infrastructure. No production decimal package is imported.

- `reference_root.go`: `certifyRootRounding` from `independent_oracle_test.go` (whole original SHA-256 `fe910b44e5a36ba2219b117253e6961347a48bdb35275b388dc06cfe33c17ec7`). Replaced testing.T failures with statuses and Decimal with validated plain state; added independent input-cost guard. Exact midpoint inequalities and odd-tie rejection are retained.

- `reference_transcendental.go`: rationalInterval/oracleLogMantissa/oracleLog/oracleRatio/oracleExp/independentlyRoundedTranscendental from the same fixed `independent_oracle_test.go` and SHA-256 above. Removed testing.T, return nil on unsupported/unproven; rational series, explicit tail bounds, reductions and 32/64/128/256 term budgets unchanged. Protocol validates candidate state and limits series precision to 1000 (exact ln/log of one and exp of zero use elementary identities through 131072) for this expensive oracle; uncertified is a failure, never a pass.
