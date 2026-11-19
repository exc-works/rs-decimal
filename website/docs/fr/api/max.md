---
locale: fr
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
translation_provenance: en-US-french-reviewed
api_id: max
---
# max

Copier la plus grande de deux valeurs décimales avec une fonction libre.

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## Paramètres, résultat et remarques

Emprunte a et b : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité numérique, conserve l’échelle et l’état d’initialisation de a. La sémantique est celle de Decimal::max.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
