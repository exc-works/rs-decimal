---
locale: de
source_revision: 6976205af89f0a7bb675a6147933c2fe624e8c9d96703f7f354c0cb5ca52f755
translation_provenance: en-US-german-reviewed
api_id: reexport::Decimal
---
# reexport::Decimal

Decimal aus der rs_decimal-Crate-Wurzel importieren.

```rust
pub use decimal::Decimal as Decimal
```

## Übliche Verwendung und Semantik

Verwenden Sie rs_decimal::Decimal; dies ist der öffentliche Importpfad desselben Elements, keine zusätzliche numerische Implementierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
