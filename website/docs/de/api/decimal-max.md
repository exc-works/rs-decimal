---
locale: de
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
translation_provenance: en-US-german-reviewed
api_id: Decimal::max
---
# Decimal::max

Den größeren von zwei Zahlenwerten kopieren.

```rust
fn max (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei Gleichheit bleiben Skala und Initialisierung des ersten Operanden erhalten. In Rust wählt Decimal::max(&left, &right) diese ausleihende inhärente Methode explizit; Ord::max übernimmt den Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
