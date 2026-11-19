---
locale: fr
source_revision: 6a03ad3f2e6f0755e57eb89608d0891566bd0e9c4b875afd8935ad4e0e27dd6d
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Utiliser l’implémentation Rust Default::default.

```rust
fn default () -> Self
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets de l’API JavaScript suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
