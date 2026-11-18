---
locale: en-GB
source_revision: ae60d30f8cb4e889e4eaa2b09d621ab4ef521d15824134a59e47eda518356ff3
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl From < isize >"
---
# Decimal::impl From < isize >

Convert isize exactly with Rust From.

```rust
impl From<isize> for Decimal
```

## Normal usage and caveats

Consumes isize and returns an initialized scale-zero Decimal. No floating-point intermediary or rounding is used; the conversion is infallible for the source integer type. Zero is initialized, unlike Decimal::default(). The source range is the Rust integer type range; isize/usize width depends on the native target. Browser code uses explicit named constructors or ConfigValue signed/unsigned variants; Rust From/Into syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-isize" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
