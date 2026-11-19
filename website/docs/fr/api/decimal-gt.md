---
locale: fr
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
translation_provenance: en-US-french-reviewed
api_id: Decimal::gt
---
# Decimal::gt

Déterminer si cette valeur est supérieure à une autre.

```rust
fn gt (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. Des valeurs numériquement égales donnent false, même si leurs échelles stockées diffèrent.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
