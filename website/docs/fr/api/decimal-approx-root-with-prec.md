---
locale: fr
source_revision: 3553acc8fce2edd77d63d567220e36aee7b664b18f15fee50a17c4ee358a3283
translation_provenance: en-US-french-reviewed
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

Calculer une racine de degré positif. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. degree : i64, transporté par bigint ; le degré doit être positif, sinon InvalidRoot est renvoyé. Les degrés impairs acceptent une entrée négative ; une racine de degré pair d’une valeur négative renvoie NegativeRoot. Le degré un correspond à un changement d’échelle avec arrondi au pair. prec : isize, transporté par un number entier ; choisit le nombre de décimales. Une précision négative renvoie InvalidPrecision avant les contrôles de domaine. L’arrondi est au pair ; une racine exactement à mi-chemin, comme sqrt(6.25) à la précision zéro, est arrondie à 2. Malgré le nom approx_root, Rust certifie l’arrondi par des comparaisons entières ; l’épuisement du budget de calcul ou d’itération du cœur renvoie NoConvergence. L’entrée est empruntée et inchangée ; les résultats, y compris pour le zéro par défaut, sont initialisés.

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite l’échelle de sortie à 1000. Les racines non triviales ont un degré maximal de 128, avec des contrôles préalables sur la mise à l’échelle, les puissances de Newton et les valeurs intermédiaires de certification du point médian. Zéro, les unités signées et les racines de degré un conservent les chemins rapides du cœur, tout en respectant les limites de précision de sortie. Les erreurs de domaine et de précision négative conservent le comportement de la bibliothèque ; les erreurs de ressources diffèrent de NoConvergence. Des entrées individuellement valides ne garantissent pas que toute combinaison degré/précision respecte le budget du navigateur.

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
