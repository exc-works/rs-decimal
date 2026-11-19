---
locale: fr
source_revision: 873d615adf07acd5810e5a3d2447199c951d65cd40cb07c1afb47b23535aad17
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Utiliser l’implémentation Rust Default::default.

```rust
fn default () -> Self
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets du bridge suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
