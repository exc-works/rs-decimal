---
locale: de
source_revision: 2558a21d5d46dd2598e1fc940985c24d9012b531617d6e30f108a6afe7d17531
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

## Hinweise und Grenzen

Der Wert ist ein Koeffizient, kein bereits skalierter Betrag. Negative Präzision liefert `InvalidPrecision`; Werte außerhalb von i64 sind Bridge-Parameterfehler.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
