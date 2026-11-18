---
locale: fr
source_revision: 872ef2a43e7ac5d06f4ea120672471b0a862ee6f8a70e48db9f1f9367f2518f7
translation_provenance: en-US-french-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

Modifier le coefficient non mis à l’échelle par un entier i64.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Paramètres, résultat et utilisation courante

value : i64, représenté par bigint dans le bridge. L’échelle est conservée : à l’échelle 2, ajouter ou soustraire 1 modifie le nombre de 0.01, pas de 1. Rust renvoie un nouveau Decimal. Le bridge rejette les number et les bigint hors de la plage i64.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
