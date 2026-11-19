---
locale: en-GB
source_revision: dcbc410f5dabf4a62b4cb667b765d034201be05823aeeb413148caad325fb23f
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

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
