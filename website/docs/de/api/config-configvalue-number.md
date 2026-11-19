---
locale: de
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

Konfigurationseingabe Number wählen.

```rust
Number (& 'a str)
```

## Parameter, Rückgabe und übliche Verwendung

Numerischen Quelltext ohne Gleitkommakonvertierung transportieren. Der Kern parst Dezimalsyntax; diese Variante validiert keine vollständige JSON-Grammatik. Leeres Number ist auch für nullable-Ziele ungültig.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
