---
locale: de
source_revision: 9ab9d90f4fdbf97aeb5b036e4b2c138345a66dab28af2a920054edaddf0608dd
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Mit Rust Default eine nicht initialisierte numerische Null erzeugen.

```rust
impl Default for Decimal
```

## Übliche Verwendung und Semantik

Default ist numerisch gleich einer konstruierten Null, aber is_nil() ist true. Die Bridge bietet Decimal.default(); diese Seite beschreibt den Rust-Trait selbst.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
