---
locale: fr
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
translation_provenance: en-US-french-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

Exiger une valeur non négative et renvoyer une copie initialisée.

```rust
fn must_non_negative (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie Decimal pour une entrée non négative, zéro compris. Une entrée négative déclenche un panic Rust avec une chaîne comme charge utile. En Wasm, c’est un trap, pas une DecimalError récupérable ; il termine l’exécution du playground. Lancez une nouvelle exécution pour obtenir un nouvel état Wasm. Il s’agit d’une assertion numérique, pas d’une conversion en valeur absolue ; utilisez abs pour changer le signe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
