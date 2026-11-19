---
locale: fr
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul
---
# Decimal::mul

Multiplier des décimaux avec une précision de sortie définie.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est la plus grande des échelles des opérandes, pas leur somme. Utilisez mul_exact pour conserver tous les chiffres du produit. Un mode explicite contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary lorsque des chiffres non nuls doivent être supprimés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
