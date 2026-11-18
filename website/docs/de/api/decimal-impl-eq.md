---
locale: de
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

Decimal verwenden, wo Rust vollständige Gleichheit verlangt.

```rust
impl Eq for Decimal
```

## Übliche Verwendung und Semantik

Eq ist ein Marker-Trait ohne Methoden. Gleichheit vergleicht Zahlen und ignoriert Skala und Initialisierung. Sie bedeutet keine Identitätsgleichheit von JavaScript-Objekten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
