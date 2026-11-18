---
locale: en-GB
source_revision: 2e4990840d666a1ee0727752c12bd5a6b1f796ab2824b1ffd20da5fc2e284755
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl From < i128 >"
---
# Decimal::impl From < i128 >

Convert i128 exactly with Rust From.

```rust
impl From<i128> for Decimal
```

## Normal usage and caveats

Consumes i128 and returns an initialized scale-zero Decimal. No floating-point intermediary or rounding is used; the conversion is infallible for the source integer type. Zero is initialized, unlike Decimal::default(). The source range is the Rust integer type range; isize/usize width depends on the native target. Browser code uses explicit named constructors or ConfigValue signed/unsigned variants; Rust From/Into syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i128" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
