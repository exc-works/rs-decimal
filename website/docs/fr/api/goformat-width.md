---
locale: fr
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
translation_provenance: en-US-french-reviewed
api_id: GoFormat.width
---
# GoFormat.width

Définir le champ de format width.

```rust
width: Option < usize >
```

## Paramètres, résultat et utilisation courante

La largeur minimale inclut le signe et le point décimal ; le remplissage ne tronque pas un texte plus long. None/null désactive la largeur minimale. Le formatage du cœur compte les octets de sortie.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
