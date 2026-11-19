---
locale: fr
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
translation_provenance: en-US-french-reviewed
api_id: Decimal::equal
---
# Decimal::equal

Déterminer si deux décimaux représentent le même nombre.

```rust
fn equal (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. L’échelle et l’état d’initialisation sont ignorés. En JavaScript, === compare l’identité des objets enveloppes ; appelez equal() pour l’égalité numérique de Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
