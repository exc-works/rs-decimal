---
locale: fr
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
translation_provenance: en-US-french-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

Comparer deux valeurs numériques avec un résultat à trois états.

```rust
fn cmp (& self , other : & Self) -> i32
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal (Decimal en JavaScript). Renvoie i32/number : -1, 0 ou 1. Des échelles et états d’initialisation différents ne modifient pas l’égalité numérique. Il s’agit de la méthode inhérente, et non de Ord::cmp qui renvoie Ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
