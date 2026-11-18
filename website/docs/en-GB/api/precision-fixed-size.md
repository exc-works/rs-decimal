---
locale: en-GB
source_revision: 73595bceae9a4435e67075be3cf3874619847ef2c8e049d0faeae85eae4feb26
translation_provenance: en-US-reviewed-reuse
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

Read the binary precision-prefix byte length.

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## Parameters, return and normal usage

A Rust usize constant with value 4: the byte length of the big-endian scale prefix in the Go-compatible binary wire format. It is not a decimal digit count or precision limit. The browser exposes a read-only number sourced from Rust. The crate-root reexport is Rust import syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="en-GB" />

## Browser boundaries

Input must be Uint8Array, and kind an integer in 0–255. Inputs are capped at 65536 bytes before copying to Wasm. Rust checks binary scale up to 1000 and actual coefficient size up to 16384 bits. BSON strings follow the 8192-byte text limit, and Decimal128 exponent expansion is also [resource-checked](../resource-policy). These limits can be narrower than the native library. Checks precede receiver mutation; core and resource errors remain distinct, and moved handles cannot be reused.

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
