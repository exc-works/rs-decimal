---
locale: fr
source_revision: f893b87d46c7c5082ca48cb1eae39734e63ca91a57f2f944773172a9baf78fca
translation_provenance: en-US-french-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

Définir le champ de format zero.

```rust
zero: bool
```

## Paramètres, résultat et utilisation courante

Compléter à gauche les sorties numériques avec des zéros après le signe. Ignoré si left vaut true ou si d possède une précision explicite. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
