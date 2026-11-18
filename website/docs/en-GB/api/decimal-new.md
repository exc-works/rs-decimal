---
locale: en-GB
source_revision: fe9121bf755dd82c88fc1274dc43b09d6173d99c7a22441eb84bc6a73587fa71
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new
---
# Decimal::new

Create an initialized whole number. Use this constructor for a signed 64-bit integer such as a count or an integral amount.

```rust
fn new (value : i64) -> Self
```

## Parameters and return

`value: i64` → JavaScript `bigint`. The result has scale zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="en-GB" />

## Usage notes and boundaries

Use `12n`, not a JavaScript number. Values outside i64 are rejected by the bridge; decimal fractions belong in `new_from_string`.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
