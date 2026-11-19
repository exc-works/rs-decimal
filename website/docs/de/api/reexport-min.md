---
locale: de
source_revision: f747bad3502ad5af9ea13bde6cf07e23bd90f835c390a356a17c381617af16c5
translation_provenance: en-US-german-reviewed
api_id: reexport::min
---
# reexport::min

min aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::min as min
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::min; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
