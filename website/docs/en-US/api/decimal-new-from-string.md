---
locale: en-US
source_revision: ea81b480a27af3babe08d7b00fe374cc7cd1626376f6bcd3383174c9fcd41789
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

Parse exact decimal text without passing through binary floating point. This is the usual constructor for user-entered decimal amounts.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## Parameters and return

`text: &str` → JavaScript `string`; Rust returns `Result<Decimal, DecimalError>`, while JavaScript returns a Decimal or throws the same library categories.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="en-US" />

## Usage notes

Input is trimmed and may contain a sign and exponent. Both sides of a decimal point require digits. The separator is always `.`, independent of UI language. Nonzero trailing zeros are retained, but `"0.00"` normalizes to initialized scale-zero zero. Invalid syntax is `InvalidFormat`.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
