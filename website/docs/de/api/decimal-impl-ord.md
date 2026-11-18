---
locale: de
source_revision: fc360056fa6a06d0df76e11b3c2a65adb20ebcb5dcc14e8a0ad1eefc1447a90e
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Ord
---
# Decimal::impl Ord

Dezimalzahlen mit Rust Ord numerisch ordnen.

```rust
impl Ord for Decimal
```

## Übliche Verwendung und Semantik

Ord::cmp liefert Ordering, anders als Decimal::cmp mit -1/0/1. Darstellungsunterschiede beeinflussen die Ordnung nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
