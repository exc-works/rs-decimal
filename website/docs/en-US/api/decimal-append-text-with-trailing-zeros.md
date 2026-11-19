---
locale: en-US
source_revision: 8f3a1f2c3a3bbab8fcd006696e4bbfc5801d795ba8c28546dfa164fc41913e69
api_id: Decimal::append_text_with_trailing_zeros
---
# Decimal::append_text_with_trailing_zeros

Append decimal ASCII bytes to an existing mutable buffer.

```rust
fn append_text_with_trailing_zeros (& self , dst : & mut Vec < u8 >)
```

## Parameters, return and normal usage

`dst: &mut Vec<u8>`, represented by a ByteBuffer wrapper. Rust returns (); JS returns undefined. Existing bytes are retained and the destination is mutated in place. All stored fractional zeros are retained. Decimal and buffer remain usable. A Uint8Array is copied into ByteBuffer.new; to_uint8_array returns a copy, not a Wasm memory view. Resource failure occurs before mutation and leaves the buffer unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
