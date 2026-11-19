---
locale: fr
source_revision: 7f9e2f6ed775898aae666efe4610ba891f54aadb6987e2a347de88c3c59500ea
translation_provenance: en-US-french-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

Calculer une racine carrée. Pas de paramètre degree. Une entrée négative renvoie NegativeRoot. La précision par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. L’arrondi est au pair ; une racine exactement à mi-chemin, comme sqrt(6.25) à la précision zéro, est arrondie à 2. Malgré le nom approx_root, Rust certifie l’arrondi par des comparaisons entières ; l’épuisement du budget de calcul ou d’itération du cœur renvoie NoConvergence. L’entrée est empruntée et inchangée ; les résultats, y compris pour le zéro par défaut, sont initialisés.

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="fr" />

## Méthodes associées

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
