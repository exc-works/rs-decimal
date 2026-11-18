---
locale: fr
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
translation_provenance: en-US-french-reviewed
api_id: RoundingMode
---
# RoundingMode

Choisir l’une des sept règles d’arrondi explicites.

```rust
pub enum RoundingMode
```

## Paramètres, résultat et utilisation courante

Utilisez les variantes nommées de l’énumération en Rust et les champs de RoundingMode dans le bridge. Les discriminants Rust 0–6 correspondent aux nombres du navigateur. Down va vers zéro ; Up s’éloigne de zéro ; Ceiling va vers plus l’infini. À mi-chemin, HalfUp/HalfDown/HalfEven choisissent respectivement l’éloignement de zéro, le rapprochement de zéro ou le chiffre pair. Unnecessary exige un résultat exact. Aucun réglage global implicite de l’arrondi n’est appliqué.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
