---
locale: fr
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

Construire à partir de la représentation décimale la plus courte qui permet de retrouver le flottant.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : f64, passé sous forme de number JavaScript. Cela ne restaure pas les chiffres décimaux déjà perdus dans un flottant binaire. Utilisez new_from_string pour une entrée décimale exacte. NaN et les deux infinis renvoient InvalidFormat. Le zéro négatif devient un zéro Decimal initialisé sans signe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
