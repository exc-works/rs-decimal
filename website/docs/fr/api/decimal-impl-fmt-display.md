---
locale: fr
source_revision: c94998594bf6a8005b50df98354b3549d47447f21a6664735b824dda07443935
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Afficher un décimal avec la syntaxe de formatage Rust.

```rust
impl fmt :: Display for Decimal
```

## Utilisation courante et sémantique

Utilisez format!("{}", value), println! ou to_string() ; le mécanisme de formatage fournit fmt::Formatter. Le format par défaut supprime les zéros fractionnaires finaux. Les options supplémentaires sont décrites dans le groupe de formatage ; utilisez string()/string_with_trailing_zeros() dans le bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
