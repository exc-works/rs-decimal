---
locale: fr
source_revision: 7c807773ded0adcdcd91027cab2d90a4029760f6cf1ca4fd49648d1e294b17d6
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

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
