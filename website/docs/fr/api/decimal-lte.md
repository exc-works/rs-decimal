---
locale: fr
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
translation_provenance: en-US-french-reviewed
api_id: Decimal::lte
---
# Decimal::lte

Déterminer si cette valeur est inférieure ou égale à une autre.

```rust
fn lte (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. L’égalité est numérique, indépendamment de l’échelle et de l’état d’initialisation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
