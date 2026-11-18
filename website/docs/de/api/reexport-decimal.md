---
locale: de
source_revision: 893ae85fdbe2ce4fb0a97cdee41fd9675ed135240e7df9b22b449b50279fb13c
translation_provenance: en-US-german-reviewed
api_id: reexport::Decimal
---
# reexport::Decimal

Decimal aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::Decimal as Decimal
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::Decimal; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
