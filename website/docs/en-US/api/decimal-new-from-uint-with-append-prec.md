---
locale: en-US
source_revision: b1a9d67b6db859834f3ef9cd2d0fefcd4239bbf10472aedba8e0a86a95b51eeb
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

Keep an unsigned integer’s numerical value while appending stored decimal places.

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: u64` → nonnegative `bigint`; `prec: isize` → integer `number`. The result retains the integer value at scale `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="en-US" />

## Usage notes

Negative/out-of-range values are invalid arguments. Negative precision is `InvalidPrecision`. Inspect trailing-zero output to see the appended places.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
