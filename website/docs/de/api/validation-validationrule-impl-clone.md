---
locale: de
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for validation::ValidationRule
```

## Parameter, Rückgabe und übliche Verwendung

Clone kopiert die vollständige Regel, Vorlagenmap oder Fehlerfelder. Die Kopie besitzt ihre Daten unabhängig. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
