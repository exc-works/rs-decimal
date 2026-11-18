---
locale: fr
source_revision: 49e05880780d4157422be53a7d3d5c1a0d8d4e809a77bd78ae251e8f6c7889c8
translation_provenance: en-US-french-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

Calculer une racine carrée. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. Pas de paramètre degree. Une entrée négative renvoie NegativeRoot. La précision par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. L’arrondi est au pair ; une racine exactement à mi-chemin, comme sqrt(6.25) à la précision zéro, est arrondie à 2. Malgré le nom approx_root, Rust certifie l’arrondi par des comparaisons entières ; l’épuisement du budget de calcul ou d’itération du cœur renvoie NoConvergence. L’entrée est empruntée et inchangée ; les résultats, y compris pour le zéro par défaut, sont initialisés.

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite l’échelle de sortie à 1000. Les racines non triviales ont un degré maximal de 128, avec des contrôles préalables sur la mise à l’échelle, les puissances de Newton et les valeurs intermédiaires de certification du point médian. Zéro, les unités signées et les racines de degré un conservent les chemins rapides du cœur, tout en respectant les limites de précision de sortie. Les erreurs de domaine et de précision négative conservent le comportement de la bibliothèque ; les erreurs de ressources diffèrent de NoConvergence. Des entrées individuellement valides ne garantissent pas que toute combinaison degré/précision respecte le budget du navigateur.

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
