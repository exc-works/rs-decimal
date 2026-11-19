---
locale: de
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
translation_provenance: en-US-german-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

Mit Erfolgskennzeichen in eine Ganzzahl fester Breite konvertieren.

```rust
fn uint64 (& self) -> (u64 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (u64, bool), abgebildet auf [bigint, boolean]. Ganzzahlige Werte im Bereich liefern die Ganzzahl und true. Brüche und Überlauf liefern (0, false), keinen abgeschnittenen Wert und keine Ausnahme. Negative Werte liefern ebenfalls (0, false). Die Skala allein bestimmt Ganzzahligkeit nicht: 12.00 gelingt. Prüfen Sie stets das Kennzeichen, um Fehler von gültiger Null zu unterscheiden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
