---
locale: de
source_revision: f5ea64d7e7a6135acc2f55f3a40411c6d9222e20ac49981dab203cb401fa6651
translation_provenance: en-US-german-reviewed
api_id: reexport::one
---
# reexport::one

one aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::one as one
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::one; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
