---
locale: de
source_revision: 8e2ccd4d5de92023ccad23e3b1ce732438fad5dab0ed6652057ad04ade8ca4a5
translation_provenance: en-US-german-reviewed
api_id: reexport::hundred
---
# reexport::hundred

hundred aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::hundred as hundred
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::hundred; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
