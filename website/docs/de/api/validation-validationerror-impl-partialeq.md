---
locale: de
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Die Rust-Implementierung von PartialEq verwenden.

```rust
impl PartialEq for validation::ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

ValidationError-Gleichheit vergleicht field, tag und ursprünglichen parameter gemeinsam, nicht nur übersetzte Meldungen. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
