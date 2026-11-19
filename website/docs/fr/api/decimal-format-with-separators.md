---
locale: fr
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: en-US-french-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

Grouper les chiffres entiers et choisir le séparateur décimal.

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## Paramètres, résultat et utilisation courante

thousands : char ; decimal : char. Renvoie String/string avec des groupes de trois chiffres entiers et toutes les décimales stockées. Chaque argument JavaScript doit être exactement un scalaire Unicode, y compris un caractère supplémentaire ; les substituts isolés et les chaînes de plusieurs caractères sont rejetés. NUL pour thousands désactive le regroupement ; NUL pour decimal choisit un point. Les séparateurs sont des caractères littéraux, pas des noms de paramètres régionaux.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
