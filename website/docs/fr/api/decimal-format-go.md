---
locale: fr
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
translation_provenance: en-US-french-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

Appliquer un spécificateur de format compatible Go et ses options.

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## Paramètres, résultat et utilisation courante

f : GoFormat ; renvoie Result<String, DecimalError>/string. Accepte un seul spécificateur, pas un motif printf tel que %.2f. Les spécificateurs pris en charge sont v/s/q/d/f/F/e/E/g/G/b. L’arrondi est au pair. Un spécificateur non pris en charge ou d appliqué à une valeur non entière produit un marqueur %! sous forme de texte, pas une exception. Une précision fixe dépassant la plage signée native renvoie InvalidPrecision ; les limites du navigateur sur la largeur, la précision et les allocations sont distinctes. Les champs JavaScript omis utilisent les valeurs par défaut de Rust ; les champs inconnus et les types de champs incorrects sont rejetés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
