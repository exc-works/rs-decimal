---
locale: fr
source_revision: f89bca6a9b41c5dfaa6d534efc134c1f42c007ba065d09915785753b5a161115
translation_provenance: en-US-french-reviewed
api_id: GoFormat
---
# GoFormat

Décrire une opération de formatage par un objet de champs.

```rust
pub struct GoFormat
```

## Paramètres, résultat et utilisation courante

Les champs correspondent à la structure Rust : verb, width, precision, left, plus, space, zero et alternate. GoFormat.default() dans le bridge lit les valeurs par défaut de Rust et renvoie un nouvel objet mutable ; sa modification ne définit pas un format global. width/precision utilisent null pour None et des number entiers non négatifs dans la plage wasm32 pour Some. La syntaxe des traits Rust reste uniquement native.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
