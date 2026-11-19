---
locale: fr
source_revision: 949ffe2e68cd75d5b05be8db37757052323a51952d74093deec642c235953650
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::unmarshal_text"
---
# NullDecimal::unmarshal_text

Décoder une entrée text dans un NullDecimal existant.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

bytes : &[u8]/Uint8Array ; modifie le récepteur et renvoie Result<(), DecimalError>/undefined. Analyse le texte décimal, en supprimant les espaces aux extrémités et en acceptant la syntaxe d’exposant. Une entrée vide efface la validité et réinitialise le Decimal interne ; le littéral null n’est pas un texte décimal valide. Une entrée autre que null, analysée avec succès, initialise le décimal, conserve l’échelle des valeurs numériquement non nulles et normalise le zéro analysé à l’échelle zéro. Définit valid=true. Les erreurs préservent l’état complet du récepteur. Les erreurs de décodage de la bibliothèque conservent Unmarshal et les catégories sous-jacentes ; les erreurs de ressources surviennent aussi avant la modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-text" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
