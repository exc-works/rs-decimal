---
locale: fr
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
translation_provenance: en-US-french-reviewed
api_id: ten
---
# ten

Construire l’entier décimal dix.

```rust
fn ten () -> Decimal
```

## Paramètres, résultat et remarques

Sans paramètre ; renvoie un nouveau Decimal initialisé d’échelle zéro. Utilisez rescale lorsqu’un nombre de décimales est nécessaire.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
