---
locale: fr
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

Utiliser l’implémentation native de Copy.

```rust
impl Copy for config::ConfigValue
```

## Paramètres, résultat et utilisation courante

ConfigValue est une petite énumération typée. Clone/Copy conservent les références empruntées au lieu de copier profondément leur cible. Debug fournit du texte de diagnostic, pas un format de transmission stable. La syntaxe des traits Rust natifs est testée séparément et ne constitue pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
