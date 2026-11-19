---
locale: fr
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
translation_provenance: en-US-french-reviewed
api_id: Decimal::lt
---
# Decimal::lt

Déterminer si cette valeur est inférieure à une autre.

```rust
fn lt (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. Des valeurs numériquement égales donnent false, même si leurs échelles stockées diffèrent.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
