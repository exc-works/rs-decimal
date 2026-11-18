---
locale: de
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

Dezimaltext mit Rust parse und FromStr einlesen.

```rust
impl FromStr for Decimal
```

## Übliche Verwendung und Semantik

Liefert Decimal oder DecimalError. Syntax und Nullnormalisierung entsprechen new_from_string. Der Browser ruft diese benannte Methode auf; Rust-Trait-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
