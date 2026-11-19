---
locale: fr
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Arrondir un rationnel à un nombre explicite de décimales.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &BigRational ; prec : isize non négatif ; mode : RoundingMode. L’API JavaScript utilise un objet bigint exact contenant le numérateur et le dénominateur. Contrairement à la conversion exacte, les fractions à développement infini telles que 1/3 sont acceptées. Une précision négative renvoie InvalidPrecision ; un arrondi nécessaire avec Unnecessary renvoie RoundUnnecessary. Un dénominateur nul est une entrée incorrecte de l’API JavaScript, rejetée avant Rust ; un numérateur nul renvoie un zéro initialisé à la précision prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
