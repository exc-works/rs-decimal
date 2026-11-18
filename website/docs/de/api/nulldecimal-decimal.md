---
locale: de
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
translation_provenance: en-US-german-reviewed
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

Die gespeicherte Dezimalzahl unabhängig von ihrer Gültigkeit untersuchen oder ersetzen.

```rust
decimal: Decimal
```

## Übliche Verwendung und Semantik

Rust stellt ein veränderliches Decimal-Feld bereit. Der Browser verwendet get_decimal_clone() für eine unabhängige Kopie und set_decimal(value), um einen Ersatzwert zu konsumieren. Kein Adapter ändert valid. Diese expliziten Adapter versprechen keine live aktualisierte Rust-Referenz in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
