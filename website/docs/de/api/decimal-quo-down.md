---
locale: de
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

Dezimalzahlen mit expliziter oder aus den Eingaben abgeleiteter Genauigkeit dividieren.

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Die Ausgabeskala ist die größere Operandenskala. Zwei Ganzzahleingaben ergeben daher eine Ganzzahl: 1 / 3 liefert 0. Verwenden Sie quo_with_prec für Nachkommastellen. Ein Nulldivisor liefert DivideByZero. Down schneidet Richtung null ab, nicht Richtung negative Unendlichkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
