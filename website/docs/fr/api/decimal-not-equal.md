---
locale: fr
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: en-US-french-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

Déterminer si deux décimaux représentent des nombres différents.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. C’est l’inverse de equal(), et non une comparaison des instantanés du coefficient et de l’échelle.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
