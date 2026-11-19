---
locale: fr
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

Multiplier des décimaux avec une précision de sortie définie.

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est prec décimales ; prec doit être non négatif, sinon InvalidPrecision est renvoyé. Un mode explicite contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary lorsque des chiffres non nuls doivent être supprimés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
