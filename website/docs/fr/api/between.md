---
locale: fr
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
translation_provenance: en-US-french-reviewed
api_id: between
---
# between

Déterminer si un décimal appartient à un intervalle numérique fermé.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## Paramètres, résultat et remarques

Emprunte v, lower et upper : &Decimal ; renvoie bool/boolean pour lower <= v <= upper. L’égalité ignore la représentation. Les bornes ne sont pas réordonnées : lower > upper produit false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
