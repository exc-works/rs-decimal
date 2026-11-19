---
locale: en-GB
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Encode the Go-compatible precision/Gob wire format.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## Parameters, return and normal usage

No parameters; returns `Result<Vec<u8>, DecimalError>`/Uint8Array. Initialized values strip fractional trailing zeros before writing a four-byte big-endian scale followed by the big.Int Gob version/sign byte and big-endian magnitude. This is a decimal scalar payload, not an IEEE float. Uninitialized zero encodes as empty bytes; initialized zero encodes as [0,0,0,0,2]. The returned copy can be modified without changing the Decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="en-GB" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
