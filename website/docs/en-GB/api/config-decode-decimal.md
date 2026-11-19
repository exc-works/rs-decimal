---
locale: en-GB
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
translation_provenance: en-US-reviewed-reuse
api_id: "config::decode_decimal"
---
# config::decode_decimal

Decode explicitly typed configuration values.

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## Parameters, return and normal usage

value: ConfigValue; returns `Result<Decimal, DecimalError>`/a new Decimal handle. Rust requires config. Non-nullable decoding rejects Null and invalid Nullable rather than inventing zero. Existing Decimal and valid Nullable values are cloned with full representation, including uninitialized zero. Text/Number/Bytes parse exact decimal text without stripping SQL quotes. Floats already carry binary rounding; use text or bigint to retain exact source digits. Browser variants belong to one run and borrow handles/buffers until decoding.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
