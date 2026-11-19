---
locale: en-US
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
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

<MethodExample id="helper-decimal-must-from-string" locale="en-US" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
