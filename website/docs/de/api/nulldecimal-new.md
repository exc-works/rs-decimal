---
locale: de
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: en-US-german-reviewed
api_id: NullDecimal::new
---
# NullDecimal::new

Eine Dezimalzahl in einen vorhandenen nullable-Wert verschieben.

```rust
fn new (decimal : Decimal) -> Self
```

## Übliche Verwendung und Semantik

decimal: Decimal wird konsumiert; das Ergebnis hat stets valid=true, auch bei nicht initialisierter Null. Die verschobene JavaScript-Eingabe ist nicht mehr nutzbar. Klonen Sie sie, wenn das Original verfügbar bleiben muss.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
