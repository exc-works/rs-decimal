---
locale: en-GB
source_revision: 9626ec029554d531f95257728b45431467328baf3b8968c06412f5cc58c689ec
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="basic-coefficient" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
