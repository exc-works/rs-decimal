# PostgreSQL / SQLx

Enable the `postgres` feature of `rs-decimal` together with the postgres feature and your chosen runtime/TLS features of SQLx **0.8.6**. The library only implements Type/Encode/Decode/PgHasArrayType; it does not create connections, choose a TLS policy, or execute queries. The test tool uses Tokio.

```toml
rs-decimal = { version = "0.1.0-alpha.1", features = ["postgres"] }
sqlx = { version = "0.8.6", default-features = false, features = ["postgres", "runtime-tokio"] }
```

The version above has not yet been published. Repository users should replace the first line with a local path or a specific git revision.

```rust,ignore
let amount = rs_decimal::Decimal::must_from_string("9007199254740993.00000000000000001");
let loaded: rs_decimal::Decimal = sqlx::query_scalar("SELECT $1::numeric")
    .bind(&amount).fetch_one(&mut connection).await?;
assert_eq!(amount, loaded);
```

NUMERIC is encoded/decoded exactly as big-endian base-10000 binary groups, without passing through f32/f64 or another decimal library. Server dscale is preserved, including 0.0000. Binary results, simple-query text results, NUMERIC arrays, and Option<Decimal>/NullDecimal are supported. A plain uninitialized Decimal is encoded as numeric zero and read back as initialized zero. An invalid NullDecimal/None is encoded as SQL NULL; decoding NULL into plain Decimal fails, while decoding into NullDecimal produces the invalid default state. The database does not store Rust's initialization flag.

Input scale > 16383 returns InvalidPrecision; more than 131072 integer digits returns Overflow. NaN/Infinity and malformed wire data are rejected. Checks finish before writing to the argument buffer, so errors append no partial payload. The maximum legal dense value with 36,864 base-10000 groups is supported; the group count is treated as PostgreSQL's unsigned 16-bit field. Only NUMERIC/NUMERIC[] type compatibility is claimed. float8/int8 columns are not automatically treated as NUMERIC; cast them explicitly in queries.

Use a NUMERIC column without declared precision/scale to preserve input scale. For NUMERIC(p,s), the server rounds and checks range according to the column declaration, which differs from Rust arithmetic's default HalfEven rule. Range and column semantics follow the [PostgreSQL NUMERIC documentation](https://www.postgresql.org/docs/18/datatype-numeric.html). SQLx type interfaces follow the pinned 0.8.6 source; transitive dependencies are locked to versions that compile with Rust 1.85. Rerun the MSRV gate after updating the lockfile.

Live validation command:

```sh
DATABASE_URL=postgres://postgres@127.0.0.1:55439/postgres \
  cargo test --locked -p decimal-postgres-tests -- --ignored
```

This test requires a dedicated server and creates only connection-local temporary tables. It fails when DATABASE_URL is missing instead of silently skipping. Regular workspace tests mark it ignored; a separate CI postgres job supplies PostgreSQL 18 and runs it explicitly. On 2026-11-06, an isolated local PostgreSQL 18 container passed tests for values beyond f64 precision, the maximum dense value with 147,455 digits, scale=16383, out-of-range encoding rejection, text/binary zero scale, NULL, valid/invalid NullDecimal, arrays, and non-finite value rejection.
