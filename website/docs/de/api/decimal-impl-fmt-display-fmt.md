---
locale: de
source_revision: ed7eae766562aad92ba36a1a91ecd3051f3e73486fdc4f475516ea974adac927
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Eine Dezimalzahl mit Rust-Formatierungssyntax ausgeben.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Übliche Verwendung und Semantik

Verwenden Sie format!("{}", value), println! oder to_string(); fmt::Formatter wird vom Formatierungsmechanismus bereitgestellt. Die Standardformatierung entfernt nachgestellte Nachkommanullen. Weitere Formatierungsflags behandelt die Formatierungsgruppe; die Bridge bietet string()/string_with_trailing_zeros().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
