---
locale: de
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: en-US-german-reviewed
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

Den von Decimals FromStr-Implementierung zurückgegebenen Fehlertyp benennen.

```rust
type Err = DecimalError
```

## Übliche Verwendung und Semantik

Der zugehörige Typ `<Decimal as FromStr>::Err` ist `DecimalError`. Ungültiger Text liefert diesen Fehler, keinen Panic. Seine Fehlerarten unterscheiden ungültige Syntax von anderen Fehlern. Dies ist ein Rust-Typalias, kein JavaScript-Konstruktor.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
