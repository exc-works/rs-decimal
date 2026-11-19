---
locale: fr
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: en-US-french-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

Définir le champ de format alternate.

```rust
alternate: bool
```

## Paramètres, résultat et utilisation courante

Conserver un point décimal pour f/F/e/E et les zéros finaux du format général pour g/G. Cela ne signifie pas un changement de base numérique. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
