---
locale: de
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo
---
# Decimal::quo

Dezimalzahlen mit expliziter oder aus den Eingaben abgeleiteter Genauigkeit dividieren.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Die Ausgabeskala ist die größere Operandenskala. Zwei Ganzzahleingaben ergeben daher eine Ganzzahl: 1 / 3 liefert mit Down den Wert 0, mit Up dagegen 1. Verwenden Sie quo_with_prec für Nachkommastellen. Ein Nulldivisor liefert DivideByZero. mode steuert die Rundung; Unnecessary liefert bei unexaktem Ergebnis RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
