---
locale: en-GB
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
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

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
