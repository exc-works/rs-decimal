---
locale: de
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

Numerische Gleichheit mit Rust-Operatoren oder PartialEq vergleichen.

```rust
fn eq (& self , other : & Self) -> bool
```

## Übliche Verwendung und Semantik

eq leiht beide Operanden aus und liefert bool. 12.00 == 12.0. JavaScript verwendet equal(); === prüft Wrapperidentität.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
