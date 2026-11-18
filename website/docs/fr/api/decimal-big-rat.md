---
locale: fr
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
translation_provenance: en-US-french-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

Lire la valeur rationnelle exacte réduite.

```rust
fn big_rat (& self) -> BigRational
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie BigRational. Le bridge renvoie un instantané immuable {numerator: bigint, denominator: bigint}, normalisé avec un dénominateur positif. L’échelle décimale et les zéros finaux ne sont pas conservés comme métadonnées rationnelles ; zéro devient 0/1. L’entrée est empruntée et inchangée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
