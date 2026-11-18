---
locale: fr
source_revision: ed7eae766562aad92ba36a1a91ecd3051f3e73486fdc4f475516ea974adac927
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Afficher un décimal avec la syntaxe de formatage Rust.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Utilisation courante et sémantique

Utilisez format!("{}", value), println! ou to_string() ; le mécanisme de formatage fournit fmt::Formatter. Le format par défaut supprime les zéros fractionnaires finaux. Les options supplémentaires sont décrites dans le groupe de formatage ; utilisez string()/string_with_trailing_zeros() dans le bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
