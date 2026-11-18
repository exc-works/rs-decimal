---
locale: en-GB
source_revision: 21a2321ebb07dad0e89ac689f0921883a544f6575bbd19e235e51c9aee9deb73
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Select Float64 configuration input.

```rust
Float64 (f64)
```

## Parameters, return and normal usage

Carry binary64 through the core shortest decimal conversion. Nonfinite values return Unmarshal; digits already lost to floating-point rounding cannot be recovered.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
