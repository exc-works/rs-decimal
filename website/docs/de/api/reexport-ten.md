---
locale: de
source_revision: 7a73d9c8a38938796f2eacbe88712430716dafd16a136172a660bfe237d37db2
translation_provenance: en-US-german-reviewed
api_id: reexport::ten
---
# reexport::ten

ten aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::ten as ten
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::ten; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
