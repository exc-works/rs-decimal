---
locale: en-US
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

Encode NullDecimal as text bytes.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing canonical ASCII decimal text. Valid values encode canonical decimal text; invalid values encode empty bytes, not the word null. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
