---
locale: en-GB
source_revision: 97e436f7da1b6009e290487900a95055ccb9df22c09952a8d25d64cf66caf012
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl From < BigInt >::from"
---
# Decimal::impl From < BigInt >::from

Convert BigInt exactly with Rust From.

```rust
fn from (value : BigInt) -> Self
```

## Normal usage and caveats

Consumes BigInt and returns an initialized scale-zero Decimal. No floating-point intermediary or rounding is used; the conversion is infallible for the source integer type. Zero is initialized, unlike Decimal::default(). BigInt ownership moves into the decimal. Browser code uses explicit named constructors or ConfigValue signed/unsigned variants; Rust From/Into syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint-from" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
