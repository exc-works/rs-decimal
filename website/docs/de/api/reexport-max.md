---
locale: de
source_revision: 24d7fa4a2221c5f03dbc4893af4fe9b8c3b42496692a1b9ed6c8a03465745caf
translation_provenance: en-US-german-reviewed
api_id: reexport::max
---
# reexport::max

max aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::max as max
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::max; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
