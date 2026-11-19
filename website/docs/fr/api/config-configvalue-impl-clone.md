---
locale: fr
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

Utiliser l’implémentation native de Clone.

```rust
impl Clone for config::ConfigValue
```

## Paramètres, résultat et utilisation courante

ConfigValue est une petite énumération typée. Clone/Copy conservent les références empruntées au lieu de copier profondément leur cible. Debug fournit du texte de diagnostic, pas un format de transmission stable. La syntaxe des traits Rust natifs est testée séparément et ne constitue pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
