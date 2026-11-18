---
locale: en-GB
source_revision: 9028fbd5554cd396883d5b8bb4b06efa13e22d1b025f5ba855a44ab02e548531
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
