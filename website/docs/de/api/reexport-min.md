---
locale: de
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
translation_provenance: en-US-german-reviewed
api_id: reexport::min
---
# reexport::min

min aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::min as min
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::min; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung. JavaScript hat keine Rust-use-Syntax; siehe die entsprechende Bridge-Referenz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
