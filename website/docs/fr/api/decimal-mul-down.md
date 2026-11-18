---
locale: fr
source_revision: 968f32ddfefb50259ee32498127a1ae440b4c11919fa84ceb9ba802a6f23f0ec
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

Multiplier des décimaux avec une précision de sortie définie.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est la plus grande des échelles des opérandes, pas leur somme. Utilisez mul_exact pour conserver tous les chiffres du produit. Down tronque vers zéro, y compris pour les résultats négatifs.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
