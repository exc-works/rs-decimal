---
locale: de
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
translation_provenance: en-US-german-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

Den vorzeichenbehafteten Rest einer Richtung null abgeschnittenen Division zurückgeben.

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Der Rest hat das Vorzeichen des Dividenden und die größere Operandenskala; dies ist kein nichtnegativer euklidischer Modulo. Ein Nulldivisor liefert DivideByZero. Eingaben werden ausgeliehen. quo_rem liefert Quotient und Rest gemeinsam.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
