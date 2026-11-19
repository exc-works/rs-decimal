---
locale: de
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
translation_provenance: en-US-german-reviewed
api_id: Decimal::float32
---
# Decimal::float32

Direkt nach IEEE binary32 konvertieren und Exaktheit melden.

```rust
fn float32 (& self) -> (f32 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert (f32, bool), abgebildet auf [number, exact]. Gleichstände werden zur geraden Zahl gerundet. Das Kennzeichen bedeutet exakte Darstellung der Dezimalzahl, nicht bloßen Konvertierungserfolg: 0.1 ist unexakt. Überlauf liefert vorzeichenbehaftete Unendlichkeit mit false; Unterlauf kann vorzeichenbehaftete Null liefern. Die Rundung erfolgt direkt nach binary32, nicht zuerst nach binary64. Das Beispiel zeichnet IEEE-Bits auf, damit JSON Sonderwerte nicht verbirgt. Playground-print kennzeichnet auch NaN, Unendlichkeiten und negative Null mit ihren übertragenen binary64-Bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
