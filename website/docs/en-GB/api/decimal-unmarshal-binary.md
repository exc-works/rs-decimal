---
locale: en-GB
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Replace a decimal from a Go-compatible binary scalar.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

bytes: &[u8]/Uint8Array; mutates the receiver and returns `Result<(), DecimalError>`/undefined. The first four bytes encode big-endian scale, followed by Gob header 2 (positive) or 3 (negative) and magnitude. Successful decoding preserves encoded scale. Empty input creates initialized zero, unlike default uninitialized zero. A four-byte prefix without a Gob payload is also accepted as initialized zero at that scale. Short prefixes, invalid versions and scale above 131072 return core Unmarshal errors. Errors preserve the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="en-GB" />

## Browser boundaries

Input must be Uint8Array, and kind an integer in 0–255. Inputs are capped at 65536 bytes before copying to Wasm. Rust checks binary scale up to 1000 and actual coefficient size up to 16384 bits. BSON strings follow the 8192-byte text limit, and Decimal128 exponent expansion is also [resource-checked](../resource-policy). These limits can be narrower than the native library. Checks precede receiver mutation; core and resource errors remain distinct, and moved handles cannot be reused.

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
