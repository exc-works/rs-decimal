---
locale: de
source_revision: ffab3b21dcd7c80e0222f2cd0457116e79f56aafc95fc2ed3c796e423b9661cc
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

Eine rationale Zahl in eine exakte endliche Dezimalzahl konvertieren.

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigRational; die JavaScript-API akzeptiert {numerator: bigint, denominator: bigint}. Der Nenner darf nicht null sein, sonst entsteht ein JavaScript-Parameterfehler. Rust BigRational normalisiert Vorzeichen und gemeinsame Faktoren. Nur gekürzte Nenner mit den Primfaktoren 2 und 5 ergeben endliche Dezimalzahlen; andere Faktoren liefern InvalidFormat. Das Ergebnis ist initialisiert. Browserprüfungen greifen nach Kürzung und begrenzen Ausgabeskala/Koeffizientenwachstum vor der Kernkonvertierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
