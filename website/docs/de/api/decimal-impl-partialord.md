---
locale: de
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

Dezimalzahlen mit der partiellen Ordnung von Rust vergleichen.

```rust
impl PartialOrd for Decimal
```

## Übliche Verwendung und Semantik

Alle Decimal-Werte sind vergleichbar; partial_cmp liefert daher Some(Ordering). Die Standardnull ist mit einer konstruierten Null vergleichbar. Unterschiedliche Skalen führen nicht zu None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
