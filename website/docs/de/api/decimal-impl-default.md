---
locale: de
source_revision: 23ce03e01f179e79048016168712d7a3101de4dd03467bd8aefb98f6fd47b269
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Mit Rust Default eine nicht initialisierte numerische Null erzeugen.

```rust
impl Default for Decimal
```

## Übliche Verwendung und Semantik

Default ist numerisch gleich einer konstruierten Null, aber is_nil() ist true. Die JavaScript-API bietet Decimal.default(); diese Seite beschreibt den Rust-Trait selbst.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
