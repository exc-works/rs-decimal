---
locale: fr
source_revision: 17e780c5fcd60ccd7701083f059fdd79ff3607aa9dff1b3dfa127b396f366781
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

Construire à partir de la représentation décimale la plus courte qui permet de retrouver le flottant.

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : f32, passé sous forme de number JavaScript. Le bridge réduit d’abord binary64 à binary32 : 16777217 devient donc 16777216. Cela ne restaure pas les chiffres décimaux déjà perdus dans un flottant binaire. Utilisez new_from_string pour une entrée décimale exacte. NaN et les deux infinis renvoient InvalidFormat ; un f64 fini qui déborde lors de la réduction en f32 renvoie aussi InvalidFormat. Le zéro négatif devient un zéro Decimal initialisé sans signe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
