---
locale: fr
source_revision: 53a165ed47764f5f661387617a1b68ba2abdefa181cac1cde270d551da87797c
translation_provenance: en-US-french-reviewed
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

Calculer une racine carrée. Pas de paramètre degree. Une entrée négative renvoie NegativeRoot. prec : isize, transporté par un number entier ; choisit le nombre de décimales. Une précision négative renvoie InvalidPrecision avant les contrôles de domaine. L’arrondi est au pair ; une racine exactement à mi-chemin, comme sqrt(6.25) à la précision zéro, est arrondie à 2. Malgré le nom approx_root, Rust certifie l’arrondi par des comparaisons entières ; l’épuisement du budget de calcul ou d’itération du cœur renvoie NoConvergence. L’entrée est empruntée et inchangée ; les résultats, y compris pour le zéro par défaut, sont initialisés.

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="fr" />

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
