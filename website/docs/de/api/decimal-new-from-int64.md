---
locale: de
source_revision: 9ec084e5d70a80123e6b5391a460c2d09074b7e750d373283106219a5cde28ed
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

Eine Dezimalzahl aus einem vorzeichenbehafteten 64-Bit-Koeffizienten und einer Nachkommastellenzahl erzeugen. Zahlenwert und Zustand verhalten sich wie bei `new_with_prec`.

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: i64` → `bigint`; `prec: isize` → ganzzahlige `number`. Liefert `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="de" />

## Hinweise zur Verwendung

Der Wert ist ein Koeffizient, kein bereits skalierter Betrag. Negative Präzision liefert `InvalidPrecision`; Werte außerhalb von i64 sind Bridge-Parameterfehler.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
