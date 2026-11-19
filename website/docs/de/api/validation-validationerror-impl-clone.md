---
locale: de
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for validation::ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

Clone kopiert die vollständige Regel, Vorlagenmap oder Fehlerfelder. Die Kopie besitzt ihre Daten unabhängig. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
