---
locale: fr
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Encoder Decimal en octets json.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Vec<u8>`/Uint8Array contenant des octets JSON. Rust exige la fonctionnalité json, activée dans le build du navigateur. Les Decimal initialisés sont encodés en chaînes canoniques entre guillemets ; le zéro par défaut non initialisé est encodé en JSON null. Les octets renvoyés sont une copie indépendante et la lecture ne modifie pas l’état.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
