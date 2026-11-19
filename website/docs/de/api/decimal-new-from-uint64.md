---
locale: de
source_revision: c4fc0f977bb46b55b53465df0296b049aa252fbfa98d47634f13b46fd3108bd0
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

Eine Dezimalzahl aus einem vorzeichenlosen 64-Bit-Koeffizienten erzeugen. Geeignet für eine nichtnegative Ganzzahlquelle, deren Nachkommastellen separat angegeben werden.

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: u64` → nichtnegatives `bigint`; `prec: isize` → ganzzahlige `number`. Liefert `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="de" />

## Hinweise zur Verwendung

Negative Werte und Werte über u64::MAX werden vor Rust abgewiesen. Negative Präzision ist der Bibliotheksfehler `InvalidPrecision`. Eine JavaScript number kann nicht den gesamten u64-Bereich sicher darstellen.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
