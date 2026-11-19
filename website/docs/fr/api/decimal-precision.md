---
locale: fr
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
translation_provenance: en-US-french-reviewed
api_id: Decimal::precision
---
# Decimal::precision

Lire le nombre de décimales stockées dans une valeur.

```rust
fn precision (& self) -> isize
```

## Paramètres, résultat et remarques

Renvoie un isize (un number entier dans le navigateur). Il s’agit de l’échelle, pas du nombre de chiffres significatifs. Les zéros finaux sont comptés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
