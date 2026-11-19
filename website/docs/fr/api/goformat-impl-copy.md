---
locale: fr
source_revision: 95811018cff564b633ac5b58792bd265ffa12dcefa76e1d1dd9c8c151efed3fd
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Copy
---
# GoFormat::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets de l’API JavaScript suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-copy" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
