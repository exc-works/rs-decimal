---
locale: fr
source_revision: 5f4867a89a40d93f24e8c716766cacdffb3d62078bd63369d4dd03f9f561ebeb
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

Utiliser l’implémentation native de Debug.

```rust
impl Debug for config::ConfigValue
```

## Paramètres, résultat et utilisation courante

ConfigValue est une petite énumération typée. Clone/Copy conservent les références empruntées au lieu de copier profondément leur cible. Debug fournit du texte de diagnostic, pas un format de transmission stable. La syntaxe des traits Rust natifs est testée séparément et ne constitue pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
