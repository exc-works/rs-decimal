---
locale: en-GB
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::big_int
---
# Decimal::big_int

Copy the exact unscaled integer coefficient.

```rust
fn big_int (& self) -> BigInt
```

## Parameters, return and usage notes

Rust returns an independent BigInt; the bridge returns bigint. This includes trailing zeros, unlike a truncating integer conversion. The example renders bigint as text for JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
