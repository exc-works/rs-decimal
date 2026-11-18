---
locale: de
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
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

## Hinweise und Grenzen

Negative Werte und Werte über u64::MAX werden vor Rust abgewiesen. Negative Präzision ist der Bibliotheksfehler `InvalidPrecision`. Eine JavaScript number kann nicht den gesamten u64-Bereich sicher darstellen.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
