---
locale: fr
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: en-US-french-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Extraire la partie entière en tronquant vers zéro.

```rust
fn int_part (& self) -> BigInt
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie BigInt et le bridge bigint, sans conversion flottante. -12.75 donne -12, et non floor(-12.75) = -13. Le zéro par défaut donne zéro. L’exemple sérialise l’entier sous forme de chaîne pour préserver sa précision arbitraire dans JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
