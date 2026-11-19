---
locale: en-GB
source_revision: 1f2a665c28ead224f75ed902dd02b6b36626040f13db084f643088b5ff82188f
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="en-GB" />

## Usage notes

Negative/out-of-range values are bridge parameter errors. Negative precision is `InvalidPrecision`. Inspect trailing-zero output to see the appended places.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
