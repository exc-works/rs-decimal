---
locale: de
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

Die native Clone-Implementierung verwenden.

```rust
impl Clone for config::ConfigValue
```

## Parameter, Rückgabe und übliche Verwendung

ConfigValue ist ein kleines typisiertes Enum. Clone/Copy erhalten ausgeliehene Referenzen, statt deren Ziel tief zu kopieren. Debug ist Diagnosetext, kein stabiles Übertragungsformat. Native Rust-Trait-Syntax wird separat getestet und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
