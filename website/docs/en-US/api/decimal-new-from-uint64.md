---
locale: en-US
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

Create a decimal from an unsigned 64-bit coefficient. Use it for a nonnegative integral source whose decimal places are supplied separately.

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: u64` → nonnegative `bigint`; `prec: isize` → integer `number`. Returns `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="en-US" />

## Usage notes and boundaries

Negative values and values above u64::MAX are rejected before entering Rust. Negative precision is the library error `InvalidPrecision`. A JavaScript number cannot safely carry the full u64 range.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
