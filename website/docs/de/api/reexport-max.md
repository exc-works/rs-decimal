---
locale: de
source_revision: 0db8fa19358a4f2f8842ab2f8da65e8e108f7914cbdecf8f41f654ca5171b70c
translation_provenance: en-US-german-reviewed
api_id: reexport::max
---
# reexport::max

max aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::max as max
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::max; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
