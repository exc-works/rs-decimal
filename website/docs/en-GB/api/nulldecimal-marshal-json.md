---
locale: en-GB
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

Encode NullDecimal as json bytes.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing JSON bytes. Rust requires the json feature; the browser build enables it. Invalid nullable values encode as JSON null; valid values are quoted canonical decimals, even when the inner Decimal is uninitialized. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="en-GB" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
