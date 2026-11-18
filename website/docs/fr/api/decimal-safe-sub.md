---
locale: fr
source_revision: 8969e894369c10d2e27ead76f07916d45a77105ffad604c0d4f46fcf59b84751
translation_provenance: en-US-french-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

Soustraire exactement des décimaux en exigeant un résultat non négatif.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre. Rust renvoie Result : seul le résultat doit être non négatif, pas chaque opérande. Un résultat négatif renvoie InvalidArgument ; zéro est accepté.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
