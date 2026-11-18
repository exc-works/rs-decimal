---
locale: en-GB
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

Name the error type returned by Decimal’s FromStr implementation.

```rust
type Err = DecimalError
```

## Normal usage and semantics

The associated type `<Decimal as FromStr>::Err` is `DecimalError`. Invalid text returns this error; it does not panic. Access its error kinds to distinguish invalid syntax from other failures. This is a Rust type alias, not a JavaScript constructor.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
