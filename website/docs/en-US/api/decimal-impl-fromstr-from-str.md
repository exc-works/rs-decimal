---
locale: en-US
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

Parse decimal text using Rust parse and FromStr.

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## Normal usage and semantics

Returns Decimal or DecimalError. Syntax and zero normalization are the same as new_from_string. The browser calls that named method; Rust trait syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
