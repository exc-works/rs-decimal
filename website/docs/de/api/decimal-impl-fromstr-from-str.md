---
locale: de
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

Dezimaltext mit Rust parse und FromStr einlesen.

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## Übliche Verwendung und Semantik

Liefert Decimal oder DecimalError. Syntax und Nullnormalisierung entsprechen new_from_string. Der Browser ruft diese benannte Methode auf; Rust-Trait-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
