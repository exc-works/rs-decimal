---
locale: fr
source_revision: 31f0a432b373e9f77b1c47a8c72a2cfedcd5df17aa590c851611115cc0270629
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Debug
---
# GoFormat::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets de l’API JavaScript suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-debug" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
