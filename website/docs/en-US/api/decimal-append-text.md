---
locale: en-US
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
api_id: Decimal::append_text
---
# Decimal::append_text

Append decimal ASCII bytes to an existing mutable buffer.

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## Parameters, return and normal usage

`dst: &mut Vec<u8>`, represented by a ByteBuffer wrapper. Rust returns (); JS returns undefined. Existing bytes are retained and the destination is mutated in place. Fractional trailing zeros are removed. Decimal and buffer remain usable. A Uint8Array is copied into ByteBuffer.new; to_uint8_array returns a copy, not a Wasm memory view. Resource failure occurs before mutation and leaves the buffer unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
