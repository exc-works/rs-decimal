---
locale: en-GB
source_revision: 742bab219fea4dd88e3233aac7a28750ce01392539e76e100af497e171dcc4c9
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::big_rat
---
# Decimal::big_rat

Read the exact reduced rational value.

```rust
fn big_rat (& self) -> BigRational
```

## Parameters, return and normal usage

No parameters; Rust returns BigRational. The JavaScript API returns an immutable {numerator: bigint, denominator: bigint} snapshot, normalized with a positive denominator. Decimal scale and trailing zeros do not survive as rational metadata; zero becomes 0/1. The input is borrowed and unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
