---
locale: en-GB
source_revision: f7f692709bde775f34d7f5feb756c59944ff77c763bae1798b2b4306d9770c41
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::big_int
---
# Decimal::big_int

Copy the exact unscaled integer coefficient.

```rust
fn big_int (& self) -> BigInt
```

## Parameters, return and usage notes

Rust returns an independent BigInt; JavaScript returns bigint. This includes trailing zeros, unlike a truncating integer conversion. The example renders bigint as text for JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
