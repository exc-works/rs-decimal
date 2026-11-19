---
locale: de
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

Die native Copy-Implementierung verwenden.

```rust
impl Copy for config::ConfigValue
```

## Parameter, Rückgabe und übliche Verwendung

ConfigValue ist ein kleines typisiertes Enum. Clone/Copy erhalten ausgeliehene Referenzen, statt deren Ziel tief zu kopieren. Debug ist Diagnosetext, kein stabiles Übertragungsformat. Native Rust-Trait-Syntax wird separat getestet und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
