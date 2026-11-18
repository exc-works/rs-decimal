---
locale: fr
source_revision: 75750ddb09ce29d94b77a840b238d39bf569cfb662c01d997d14bb79d57564c3
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Arrondir un rationnel à un nombre explicite de décimales.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &BigRational ; prec : isize non négatif ; mode : RoundingMode. Le bridge utilise un objet bigint exact contenant le numérateur et le dénominateur. Contrairement à la conversion exacte, les fractions à développement infini telles que 1/3 sont acceptées. Une précision négative renvoie InvalidPrecision ; un arrondi nécessaire avec Unnecessary renvoie RoundUnnecessary. Un dénominateur nul est une entrée incorrecte du bridge, rejetée avant Rust ; un numérateur nul renvoie un zéro initialisé à la précision prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
