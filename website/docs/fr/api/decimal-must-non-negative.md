---
locale: fr
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
translation_provenance: en-US-french-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

Exiger une valeur non négative et renvoyer une copie initialisée.

```rust
fn must_non_negative (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie Decimal pour une entrée non négative, zéro compris. Une entrée négative déclenche un panic Rust avec une chaîne comme charge utile. En Wasm, c’est un trap, pas une DecimalError récupérable ; il termine l’exécution du playground. Lancez une nouvelle exécution pour obtenir un nouvel état Wasm. Il s’agit d’une assertion numérique, pas d’une conversion en valeur absolue ; utilisez abs pour changer le signe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
