---
locale: de
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: en-US-german-reviewed
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

In Rust Gültigkeit und gespeicherte Dezimalzahl gemeinsam klonen.

```rust
impl Clone for NullDecimal
```

## Übliche Verwendung und Semantik

Klonen erhält verborgene Werte auch bei valid=false. Dies ist ein nativer Rust-Trait, keine gemeinsame Referenz, die spätere Änderungen verfolgt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
