---
locale: en-GB
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="constructor-new-from-int" locale="en-GB" />

## Usage notes

The browser target is wasm32, so the accepted range is -2147483648 through 2147483647 even when native Rust runs on a 64-bit machine. Fractions, NaN and infinity are parameter errors.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
