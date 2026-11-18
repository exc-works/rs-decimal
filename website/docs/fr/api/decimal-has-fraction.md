---
locale: fr
source_revision: 4f94d3a29039b5504e83148308232156a8a140f150fc1e38d0956e8fb963c92b
translation_provenance: en-US-french-reviewed
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Examiner la partie fractionnaire numérique.

```rust
fn has_fraction (& self) -> bool
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie bool/boolean. Le test porte sur la valeur numérique, pas sur une échelle égale à zéro : 12.00 est un entier. Le zéro par défaut est aussi un entier sans partie fractionnaire. Ces requêtes n’initialisent ni ne modifient l’entrée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
