---
locale: fr
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
translation_provenance: en-US-french-reviewed
api_id: min
---
# min

Copier la plus petite de deux valeurs décimales avec une fonction libre.

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## Paramètres, résultat et remarques

Emprunte a et b : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité numérique, conserve l’échelle et l’état d’initialisation de a. La sémantique est celle de Decimal::min.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
