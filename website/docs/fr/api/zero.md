---
locale: fr
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
translation_provenance: en-US-french-reviewed
api_id: zero
---
# zero

Construire un zéro initialisé d’échelle zéro.

```rust
fn zero () -> Decimal
```

## Paramètres, résultat et remarques

Sans paramètre ; renvoie un nouveau Decimal. Contrairement à Decimal::default(), le résultat est initialisé, bien que les deux soient numériquement égaux.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
