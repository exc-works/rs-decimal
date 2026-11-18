---
locale: fr
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: en-US-french-reviewed
api_id: NullDecimal::new
---
# NullDecimal::new

Déplacer un décimal dans une valeur nullable présente.

```rust
fn new (decimal : Decimal) -> Self
```

## Utilisation courante et sémantique

decimal : Decimal est consommé ; le résultat a toujours valid=true, même si decimal est un zéro non initialisé. L’entrée JavaScript déplacée n’est plus utilisable. Fournissez un clone pour conserver l’original.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
