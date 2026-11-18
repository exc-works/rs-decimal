---
locale: fr
source_revision: e0f36bf4a4b64a112b7758ba3b58ccfab48769ae252409c4b72638890e0e2a0e
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

Arrondir au plus proche, en s’éloignant de zéro à mi-chemin.

```rust
HalfUp = 3
```

## Paramètres, résultat et utilisation courante

Discriminant entier 3 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
