---
locale: fr
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
translation_provenance: en-US-french-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

Définir le champ de format precision.

```rust
precision: Option < usize >
```

## Paramètres, résultat et utilisation courante

La précision dépend du spécificateur : f/F comptent les décimales, e/E les positions après le premier chiffre, g/G les chiffres significatifs (zéro est traité comme un), et d le nombre minimal de chiffres entiers. None/null conserve les règles par défaut du spécificateur ; f conserve l’échelle, e utilise six décimales.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
