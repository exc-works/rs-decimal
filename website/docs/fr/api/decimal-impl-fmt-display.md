---
locale: fr
source_revision: 97b85000fbf7a0fb6537eae68258f927a7739647e41e96de3c0f26f460f2d35a
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Afficher un décimal avec la syntaxe de formatage Rust.

```rust
impl fmt :: Display for Decimal
```

## Utilisation courante et sémantique

Utilisez format!("{}", value), println! ou to_string() ; le mécanisme de formatage fournit fmt::Formatter. Le format par défaut supprime les zéros fractionnaires finaux. Les options supplémentaires sont décrites dans le groupe de formatage ; utilisez string()/string_with_trailing_zeros() en JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
