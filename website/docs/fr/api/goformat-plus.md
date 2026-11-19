---
locale: fr
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: en-US-french-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

Définir le champ de format plus.

```rust
plus: bool
```

## Paramètres, résultat et utilisation courante

Afficher + pour les sorties numériques non négatives. Prioritaire sur space. S’applique aux spécificateurs numériques, pas à v/s/q ; les valeurs négatives conservent -. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
