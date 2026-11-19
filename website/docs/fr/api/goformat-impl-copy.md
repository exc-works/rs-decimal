---
locale: fr
source_revision: d5ce77b83bcc1f28279c44d6d96fc602e4eb59aa65aad6eb814e4e715f060309
translation_provenance: en-US-french-reviewed
api_id: GoFormat::impl Copy
---
# GoFormat::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for GoFormat
```

## Paramètres, résultat et utilisation courante

GoFormat est une petite structure de champs Copy. Clone/Copy conservent les options ; Debug affiche les champs. Default choisit v, None pour width/precision et false pour tous les indicateurs. Cette syntaxe Rust est testée nativement ; les objets du bridge suivent la sémantique ordinaire des objets de champs JavaScript et de GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-copy" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
