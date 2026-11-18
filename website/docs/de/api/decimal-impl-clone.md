---
locale: de
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

Eine Dezimalzahl unter Erhalt ihrer vollständigen Darstellung klonen.

```rust
impl Clone for Decimal
```

## Übliche Verwendung und Semantik

Clone kopiert Koeffizient, Skala und Initialisierung; JavaScript verwendet den benannten Adapter clone() statt eines Rust-Traits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
