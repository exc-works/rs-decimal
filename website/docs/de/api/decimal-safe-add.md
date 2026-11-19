---
locale: de
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
translation_provenance: en-US-german-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

Dezimalzahlen exakt addieren und ein nichtnegatives Ergebnis verlangen.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern. Rust liefert Result: Nur das Ergebnis muss nichtnegativ sein, nicht jeder Operand. Ein negatives Ergebnis liefert InvalidArgument; null ist erlaubt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
