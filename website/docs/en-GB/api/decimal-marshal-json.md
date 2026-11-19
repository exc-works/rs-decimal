---
locale: en-GB
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Encode Decimal as json bytes.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing JSON bytes. Rust requires the json feature; the browser build enables it. Initialized Decimal values encode as quoted canonical strings; default uninitialized zero encodes as JSON null. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="en-GB" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
