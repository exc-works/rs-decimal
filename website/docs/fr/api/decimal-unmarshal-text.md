---
locale: fr
source_revision: 7e0df53b59ae6c40ced288a2a324fe9ef0557e3dc38c029cf8a66e1749c4ce28
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

Décoder une entrée text dans un Decimal existant.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

bytes : &[u8]/Uint8Array ; modifie le récepteur et renvoie Result<(), DecimalError>/undefined. Analyse le texte décimal, en supprimant les espaces aux extrémités et en acceptant la syntaxe d’exposant. Le texte vide et le littéral null ne sont pas des textes décimaux valides ; utilisez le décodage nullable lorsque le vide représente l’absence. Une entrée autre que null, analysée avec succès, initialise le décimal, conserve l’échelle des valeurs numériquement non nulles et normalise le zéro analysé à l’échelle zéro. Les erreurs préservent l’état complet du récepteur. Les erreurs de décodage de la bibliothèque conservent Unmarshal et les catégories sous-jacentes ; les erreurs de ressources surviennent aussi avant la modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="fr" />

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
