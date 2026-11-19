---
locale: fr
source_revision: 8eec4d40194be2cd158d623391729d3b1e9d3b0532af669f3499b5168782b68c
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

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
