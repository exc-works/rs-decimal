---
locale: en-GB
source_revision: 910a84a6ded31baaefbfbdf3f7b025090c5c19a248c0ddcdb4d560e598525806
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl From < usize >"
---
# Decimal::impl From < usize >

Convert usize exactly with Rust From.

```rust
impl From<usize> for Decimal
```

## Normal usage and caveats

Consumes usize and returns an initialized scale-zero Decimal. No floating-point intermediary or rounding is used; the conversion is infallible for the source integer type. Zero is initialized, unlike Decimal::default(). The source range is the Rust integer type range; isize/usize width depends on the native target. Browser code uses explicit named constructors or ConfigValue signed/unsigned variants; Rust From/Into syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-usize" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
