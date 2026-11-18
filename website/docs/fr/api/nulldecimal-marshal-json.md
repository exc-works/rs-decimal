---
locale: fr
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

Encoder NullDecimal en octets json.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Vec<u8>`/Uint8Array contenant des octets JSON. Rust exige la fonctionnalité json, activée dans le build du navigateur. Les valeurs nullables invalides sont encodées en JSON null ; les valeurs valides en décimaux canoniques entre guillemets, même si le Decimal interne n’est pas initialisé. Les octets renvoyés sont une copie indépendante et la lecture ne modifie pas l’état.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="fr" />

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
