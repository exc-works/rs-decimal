---
locale: en-US
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

Format a nullable value with Rust Display.

```rust
impl std :: fmt :: Display for NullDecimal 
```

## Parameters, return and normal usage

Display delegates to nullable string(): canonical inner text when valid and null when invalid. The formatter writes text, not a JSON token or byte buffer. format! invokes Display::fmt; this trait syntax is executed natively and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
