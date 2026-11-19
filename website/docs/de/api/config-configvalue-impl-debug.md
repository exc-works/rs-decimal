---
locale: de
source_revision: 5f4867a89a40d93f24e8c716766cacdffb3d62078bd63369d4dd03f9f561ebeb
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

Die native Debug-Implementierung verwenden.

```rust
impl Debug for config::ConfigValue
```

## Parameter, Rückgabe und übliche Verwendung

ConfigValue ist ein kleines typisiertes Enum. Clone/Copy erhalten ausgeliehene Referenzen, statt deren Ziel tief zu kopieren. Debug ist Diagnosetext, kein stabiles Übertragungsformat. Native Rust-Trait-Syntax wird separat getestet und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
