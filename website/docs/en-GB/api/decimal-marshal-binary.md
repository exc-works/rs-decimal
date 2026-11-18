---
locale: en-GB
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
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

## Browser boundaries

Input must be Uint8Array, and kind an integer in 0–255. Inputs are capped at 65536 bytes before copying to Wasm. Rust checks binary scale up to 1000 and actual coefficient size up to 16384 bits. BSON strings follow the 8192-byte text limit, and Decimal128 exponent expansion is also [resource-checked](../resource-policy). These limits can be narrower than the native library. Checks precede receiver mutation; core and resource errors remain distinct, and moved handles cannot be reused.

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
