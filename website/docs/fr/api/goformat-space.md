---
locale: fr
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
translation_provenance: en-US-french-reviewed
api_id: GoFormat.space
---
# GoFormat.space

Définir le champ de format space.

```rust
space: bool
```

## Paramètres, résultat et utilisation courante

Ajouter un espace initial aux sorties numériques non négatives lorsque plus vaut false. Les valeurs négatives conservent leur signe. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
