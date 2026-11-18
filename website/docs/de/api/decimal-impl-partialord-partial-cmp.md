---
locale: de
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

Dezimalzahlen mit der partiellen Ordnung von Rust vergleichen.

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## Übliche Verwendung und Semantik

Alle Decimal-Werte sind vergleichbar; partial_cmp liefert daher Some(Ordering). Die Standardnull ist mit einer konstruierten Null vergleichbar. Unterschiedliche Skalen führen nicht zu None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
