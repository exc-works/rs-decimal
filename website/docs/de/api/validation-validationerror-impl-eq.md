---
locale: de
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Die Rust-Implementierung von Eq verwenden.

```rust
impl Eq for validation::ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

ValidationError-Gleichheit vergleicht field, tag und ursprünglichen parameter gemeinsam, nicht nur übersetzte Meldungen. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
