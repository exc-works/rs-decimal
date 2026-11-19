---
locale: de
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

Dezimalzahlen mit expliziter oder aus den Eingaben abgeleiteter Genauigkeit dividieren.

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. prec wählt nichtnegative Nachkommastellen; negative Präzision liefert InvalidPrecision vor der Divisorprüfung. Ein Nulldivisor liefert DivideByZero. mode steuert die Rundung; Unnecessary liefert bei unexaktem Ergebnis RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
