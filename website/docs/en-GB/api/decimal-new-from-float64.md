---
locale: en-GB
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

Construct from the shortest decimal that round-trips to a float.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: f64, passed as a JS number. This does not recover decimal digits already lost to binary floating point. Use new_from_string for exact decimal input. NaN and either infinity return InvalidFormat. Negative zero becomes initialized Decimal zero without a sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
