---
locale: fr
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

Utiliser Decimal lorsque Rust exige une relation d’égalité totale.

```rust
impl Eq for Decimal
```

## Utilisation courante et sémantique

Eq est un trait marqueur sans méthode. L’égalité compare les nombres et ignore l’échelle ou l’initialisation. Elle n’implique pas l’égalité d’identité des objets JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
