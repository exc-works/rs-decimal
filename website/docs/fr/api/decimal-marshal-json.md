---
locale: fr
source_revision: ce7a1e6d51ea25aa6e22728a52c66d3bfd202ca324cd22c0d1d0545a351bced2
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

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
