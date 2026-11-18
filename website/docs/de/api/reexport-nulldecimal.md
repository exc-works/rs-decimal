---
locale: de
source_revision: 05a4d32c73f6c4d59c8e657272c093b1db00da738b0234c960526209d3e3196c
translation_provenance: en-US-german-reviewed
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

NullDecimal aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::NullDecimal as NullDecimal
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::NullDecimal; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
