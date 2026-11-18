---
locale: fr
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

Multiplier exactement des décimaux.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est la somme des échelles des opérandes ; tous les chiffres du produit sont conservés. Un dépassement de l’échelle dans le cœur renvoie Overflow. Il n’y a pas de paramètre de mode d’arrondi.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
