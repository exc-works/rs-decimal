---
locale: de
source_revision: 53c6a8acf9ea21b47dcf7e64e91f78d378746d55bbda43af84954ef21da2900f
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Eine Dezimalzahl mit Rust-Formatierungssyntax ausgeben.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Übliche Verwendung und Semantik

Verwenden Sie format!("{}", value), println! oder to_string(); fmt::Formatter wird vom Formatierungsmechanismus bereitgestellt. Die Standardformatierung entfernt nachgestellte Nachkommanullen. Weitere Formatierungsflags behandelt die Formatierungsgruppe; die JavaScript-API bietet string()/string_with_trailing_zeros().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
