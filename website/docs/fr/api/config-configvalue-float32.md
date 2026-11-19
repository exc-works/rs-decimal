---
locale: fr
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

Sélectionner une entrée de configuration Float32.

```rust
Float32 (f32)
```

## Paramètres, résultat et utilisation courante

Transporte un binary32. Rust convertit d’abord le number du navigateur en f32, puis utilise la conversion du cœur. Une entrée non finie ou un dépassement lors de cette conversion renvoie Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
