---
locale: en-US
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Encode Decimal as text bytes.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing canonical ASCII decimal text. Fractional trailing zeros are removed. Default Decimal zero prints ASCII 0. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
