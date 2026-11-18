---
locale: de
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: en-US-german-reviewed
api_id: NullDecimal
---
# NullDecimal

Eine optionale Dezimalzahl mit unabhängiger Gültigkeit und gespeichertem Wert darstellen.

```rust
pub struct NullDecimal
```

## Übliche Verwendung und Semantik

Nur valid bestimmt die Anwesenheit. Ein ungültiger nullable-Wert kann eine Dezimalzahl ungleich null behalten; ein gültiger kann nicht initialisierte Null enthalten. Das öffentliche Rust-Feld decimal wird in JavaScript über explizite Kopier-/Move-Adapter bereitgestellt. Default bedeutet abwesend.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
