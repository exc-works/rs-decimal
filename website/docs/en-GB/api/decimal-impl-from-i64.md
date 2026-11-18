---
locale: en-GB
source_revision: a85e1f50548b76565d69e68ea638117be1814bc14ac0a7bc1861f94179abfc92
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl From < i64 >"
---
# Decimal::impl From < i64 >

Convert i64 exactly with Rust From.

```rust
impl From<i64> for Decimal
```

## Normal usage and caveats

Consumes i64 and returns an initialized scale-zero Decimal. No floating-point intermediary or rounding is used; the conversion is infallible for the source integer type. Zero is initialized, unlike Decimal::default(). The source range is the Rust integer type range; isize/usize width depends on the native target. Browser code uses explicit named constructors or ConfigValue signed/unsigned variants; Rust From/Into syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i64" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
