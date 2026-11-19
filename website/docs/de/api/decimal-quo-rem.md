---
locale: de
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

Einen ganzzahligen Quotienten und einen vorzeichenbehafteten Decimal-Rest zurückgeben.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<(Decimal, Decimal), DecimalError>, abgebildet auf [quotient, remainder]. Der Quotient wird Richtung null auf Skala null abgeschnitten. Der Rest hat das Vorzeichen des Dividenden und die größere Operandenskala; self = quotient * other + remainder gilt. Jedes Ergebnis ist ein unabhängiger initialisierter Wrapper; Eingaben bleiben nutzbar. Ein Nulldivisor liefert DivideByZero vor der Skalenausrichtung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
