---
locale: fr
source_revision: 9f9019ab26936086c52db8997b79603161a6ca91ecf3ebbb2e2fc9ec02fd1a05
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Default
---
# GoFormat::impl Default

Utiliser l’implémentation Rust Default.

```rust
impl Default for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets du bridge suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
