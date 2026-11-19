---
locale: fr
source_revision: 2e472bd0598b8ec796fb5adb09d98ce669a8d1defb11e5244f124c816e8d2cd8
translation_provenance: en-US-french-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

Calculer une racine de degré positif. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. degree : i64, transporté par bigint ; le degré doit être positif, sinon InvalidRoot est renvoyé. Les degrés impairs acceptent une entrée négative ; une racine de degré pair d’une valeur négative renvoie NegativeRoot. Le degré un correspond à un changement d’échelle avec arrondi au pair. La précision par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. L’arrondi est au pair ; une racine exactement à mi-chemin, comme sqrt(6.25) à la précision zéro, est arrondie à 2. Malgré le nom approx_root, Rust certifie l’arrondi par des comparaisons entières ; l’épuisement du budget de calcul ou d’itération du cœur renvoie NoConvergence. L’entrée est empruntée et inchangée ; les résultats, y compris pour le zéro par défaut, sont initialisés.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="fr" />

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
