---
locale: de
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
translation_provenance: en-US-german-reviewed
api_id: Decimal::int64
---
# Decimal::int64

Mit Erfolgskennzeichen in eine Ganzzahl fester Breite konvertieren.

```rust
fn int64 (& self) -> (i64 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (i64, bool), abgebildet auf [bigint, boolean]. Ganzzahlige Werte im Bereich liefern die Ganzzahl und true. Brüche und Überlauf liefern (0, false), keinen abgeschnittenen Wert und keine Ausnahme. Die Skala allein bestimmt Ganzzahligkeit nicht: 12.00 gelingt. Prüfen Sie stets das Kennzeichen, um Fehler von gültiger Null zu unterscheiden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
