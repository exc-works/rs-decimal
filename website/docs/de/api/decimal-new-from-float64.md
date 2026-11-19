---
locale: de
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

Aus der kürzesten Dezimaldarstellung konstruieren, die wieder denselben Gleitkommawert ergibt.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: f64, als JS number übergeben. Bereits durch binäre Gleitkommadarstellung verlorene Dezimalziffern werden nicht wiederhergestellt. Verwenden Sie new_from_string für exakte Dezimaleingaben. NaN und beide Unendlichkeiten liefern InvalidFormat. Negative Null wird zur initialisierten Decimal-Null ohne Vorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
