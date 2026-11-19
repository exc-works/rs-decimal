---
locale: fr
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
translation_provenance: en-US-french-reviewed
api_id: hundred
---
# hundred

Construire l’entier décimal cent.

```rust
fn hundred () -> Decimal
```

## Paramètres, résultat et remarques

Sans paramètre ; renvoie un nouveau Decimal initialisé d’échelle zéro. C’est un constructeur de valeur, pas une constante mutable partagée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
