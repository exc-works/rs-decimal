---
locale: fr
source_revision: 4e5c7dfa4cd03b60430392b3a97363dc09a63cee40fa7c3bab57e19b48c4d145
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Debug
---
# GoFormat::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets du bridge suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-debug" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
