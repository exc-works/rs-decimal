---
locale: fr
source_revision: 240d5cd6140ed74d787cde34f7f3b1ea17105d751eeea756af72039c1ea346ce
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

Diviser des décimaux avec une précision explicite ou dérivée des entrées.

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. prec choisit un nombre non négatif de décimales ; une précision négative renvoie InvalidPrecision avant la vérification du diviseur. Un diviseur nul renvoie DivideByZero. mode contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary pour un résultat inexact.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="fr" />

## Limites du navigateur

Les entrées sont empruntées et restent utilisables après une réussite ou une erreur ordinaire. Les Decimal renvoyés possèdent des données indépendantes ; les résultats arithmétiques sont initialisés. Le navigateur vérifie la [politique de ressources](../resource-policy) avant d’appeler le cœur : alignement, produits intermédiaires, mise à l’échelle de la division et précision de sortie. Ces contrôles conservateurs peuvent rejeter un calcul dont le résultat final est petit. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque ; un panic termine l’exécution.

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
