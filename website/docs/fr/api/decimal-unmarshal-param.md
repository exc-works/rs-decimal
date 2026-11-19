---
locale: fr
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

Décoder une entrée param dans un Decimal existant.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

param : &str/string ; modifie le récepteur et renvoie Result<(), DecimalError>/undefined. Analyse le texte décimal, en supprimant les espaces aux extrémités et en acceptant la syntaxe d’exposant. Le texte vide et le littéral null ne sont pas des textes décimaux valides ; utilisez le décodage nullable lorsque le vide représente l’absence. Une entrée autre que null, analysée avec succès, initialise le décimal, conserve l’échelle des valeurs numériquement non nulles et normalise le zéro analysé à l’échelle zéro. Les erreurs préservent l’état complet du récepteur. Les erreurs de décodage de la bibliothèque conservent Unmarshal et les catégories sous-jacentes ; les erreurs de ressources surviennent aussi avant la modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
