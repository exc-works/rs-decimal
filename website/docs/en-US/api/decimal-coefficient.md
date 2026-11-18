---
locale: en-US
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
api_id: Decimal::coefficient
---
# Decimal::coefficient

Inspect the exact unscaled integer coefficient.

```rust
fn coefficient (& self) -> & BigInt
```

## Parameters, return and usage notes

Rust borrows &BigInt; the bridge returns an immutable bigint snapshot. It is not a live reference. The coefficient includes trailing zeros and is not the integer part of the decimal. The example renders bigint as text to preserve all digits in JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
