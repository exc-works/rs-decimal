---
locale: de
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

Numerische Gleichheit mit Rust-Operatoren oder PartialEq vergleichen.

```rust
impl PartialEq for Decimal
```

## Übliche Verwendung und Semantik

eq leiht beide Operanden aus und liefert bool. 12.00 == 12.0. JavaScript verwendet equal(); === prüft Wrapperidentität.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
