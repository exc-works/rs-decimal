---
locale: fr
source_revision: 098847ed019375f939ef5ba1a900bfb15c1c278feadb8bae9f85384666b02bf1
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::unmarshal_param"
---
# NullDecimal::unmarshal_param

Décoder une entrée param dans un NullDecimal existant.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

param : &str/string ; modifie le récepteur et renvoie Result<(), DecimalError>/undefined. Analyse le texte décimal, en supprimant les espaces aux extrémités et en acceptant la syntaxe d’exposant. Une entrée vide efface la validité et réinitialise le Decimal interne ; le littéral null n’est pas un texte décimal valide. Une entrée autre que null, analysée avec succès, initialise le décimal, conserve l’échelle des valeurs numériquement non nulles et normalise le zéro analysé à l’échelle zéro. Définit valid=true. Les erreurs préservent l’état complet du récepteur. Les erreurs de décodage de la bibliothèque conservent Unmarshal et les catégories sous-jacentes ; les erreurs de ressources surviennent aussi avant la modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-param" locale="fr" />

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
