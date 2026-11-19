---
locale: fr
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: en-US-french-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

Définir le champ de format verb.

```rust
verb: char
```

## Paramètres, résultat et utilisation courante

Choisir un spécificateur de format constitué d’un seul scalaire Unicode. Par défaut, v utilise une notation fixe compacte ; e/E utilisent la notation scientifique, q ajoute des guillemets et b affiche le coefficient binaire suivi de p-scale, et non un flottant IEEE.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
