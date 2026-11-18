---
locale: en-GB
source_revision: 425ca68f655481c858e4668ea2e1df0ddc734454c90f72d356190ed7ac4eb22e
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="en-GB" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
