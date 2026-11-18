---
locale: de
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

Dezimalzahlen mit Rust Ord numerisch ordnen.

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## Übliche Verwendung und Semantik

Ord::cmp liefert Ordering, anders als Decimal::cmp mit -1/0/1. Darstellungsunterschiede beeinflussen die Ordnung nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
