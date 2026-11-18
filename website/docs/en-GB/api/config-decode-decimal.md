---
locale: en-GB
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
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

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
