---
locale: de
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

Konfigurationseingabe Float32 wählen.

```rust
Float32 (f32)
```

## Parameter, Rückgabe und übliche Verwendung

binary32 transportieren. Rust verengt die Browser-number zuerst auf f32 und verwendet die Kernkonvertierung. Nicht endliche Eingabe oder Verengungsüberlauf liefert Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
