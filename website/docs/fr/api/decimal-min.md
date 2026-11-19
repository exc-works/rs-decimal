---
locale: fr
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
translation_provenance: en-US-french-reviewed
api_id: Decimal::min
---
# Decimal::min

Copier la plus petite de deux valeurs numériques.

```rust
fn min (& self , other : & Self) -> Self
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité, conserve l’échelle et l’état d’initialisation du premier opérande. En Rust, utilisez Decimal::min(&left, &right) pour éviter la méthode du trait Ord::min qui consomme la valeur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
