---
locale: en-GB
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

Parse trusted decimal text when invalid input is a programming error.

```rust
fn must_from_string (text : & str) -> Self
```

## Parameters, return and usage notes

text: &str/string; returns Decimal. Valid input has the same state as new_from_string. Invalid syntax panics with DecimalError in Rust; compiled Wasm traps rather than returning a recoverable library error. Prefer new_from_string for user input. An uncaught trap ends the current run; start a fresh run before using Rust again. Parameter and resource checks still reject input before calling Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="en-GB" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
