---
locale: de
source_revision: 29e91f626fd685fb0b5987fc104c1ba0e2752897194773305e3d64dcc1b16f1a
translation_provenance: en-US-german-reviewed
api_id: reexport::one
---
# reexport::one

one aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::one as one
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::one; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
