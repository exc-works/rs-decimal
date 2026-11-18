---
locale: en-US
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

Create an initialized integer from a native-width signed integer.

```rust
fn new_from_int (value : isize) -> Self
```

## Parameters and return

`value: isize` → an integer JavaScript `number`; result scale is zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="en-US" />

## Usage notes and boundaries

The browser target is wasm32, so the accepted range is -2147483648 through 2147483647 even when native Rust runs on a 64-bit machine. Fractions, NaN and infinity are parameter errors.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
