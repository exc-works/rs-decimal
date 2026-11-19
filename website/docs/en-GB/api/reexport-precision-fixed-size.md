---
locale: en-GB
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
translation_provenance: en-US-reviewed-reuse
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

Read the binary precision-prefix byte length.

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## Parameters, return and normal usage

A Rust usize constant with value 4: the byte length of the big-endian scale prefix in the Go-compatible binary wire format. It is not a decimal digit count or precision limit. The browser exposes a read-only number sourced from Rust. The crate-root reexport is Rust import syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="en-GB" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
