---
locale: fr
source_revision: 4842619da75eb5fbcf11cdce1b3ca781ba18031962688508c5f82ef39efdcf01
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo
---
# Decimal::quo

Diviser des décimaux avec une précision explicite ou dérivée des entrées.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. L’échelle de sortie est la plus grande des échelles des opérandes. Deux entrées entières produisent donc un entier : 1 / 3 donne 0. Utilisez quo_with_prec pour une sortie fractionnaire. Un diviseur nul renvoie DivideByZero. mode contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary pour un résultat inexact.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
