---
locale: fr
source_revision: 03df94c2d70888cf880122deea0e6ec382870fa1e10b3d6cb2d26b3fc0fb348a
translation_provenance: en-US-french-reviewed
api_id: Decimal::gte
---
# Decimal::gte

Déterminer si cette valeur est supérieure ou égale à une autre.

```rust
fn gte (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. L’égalité est numérique, indépendamment de l’échelle et de l’état d’initialisation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
