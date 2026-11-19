---
locale: fr
source_revision: 88974eef4784bf8f1c8b6ec4ce25d3380a2169d449467ecdb4cee7997d0acdd8
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets de l’API JavaScript suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
