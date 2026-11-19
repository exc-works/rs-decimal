---
locale: de
source_revision: d9e6c46654b05ae244bd56f7035b17e55d6f30ce2bee3665fad9be992491dd6d
translation_provenance: en-US-german-reviewed
api_id: reexport::zero
---
# reexport::zero

zero aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::zero as zero
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::zero; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
