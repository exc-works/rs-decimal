---
locale: en-GB
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

Format a nullable value with Rust Display.

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## Parameters, return and normal usage

Display delegates to nullable string(): canonical inner text when valid and null when invalid. The formatter writes text, not a JSON token or byte buffer. format! invokes Display::fmt; this trait syntax is executed natively and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="en-GB" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
