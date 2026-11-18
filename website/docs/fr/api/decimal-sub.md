---
locale: fr
source_revision: 922fb0ccad86b971140048111e8d1bdfebf3b2480d201800dddb60a3c6890884
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub
---
# Decimal::sub

Soustraire exactement des décimaux.

```rust
fn sub (& self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre. La méthode Rust est infaillible ; les limites d’allocation du navigateur peuvent néanmoins produire une erreur de ressources.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
