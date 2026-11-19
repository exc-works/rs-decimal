---
locale: en-US
source_revision: ade28fb6fdcde851797c02557b97ea1da6938690477fe011590975bc67fbbf2e
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

Parse exact decimal text without passing through binary floating point. This is the usual constructor for user-entered decimal amounts.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## Parameters and return

`text: &str` → JavaScript `string`; Rust returns `Result<Decimal, DecimalError>`, while the bridge returns a Decimal or throws the same library categories.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="en-US" />

## Usage notes

Input is trimmed and may contain a sign and exponent. Both sides of a decimal point require digits. The separator is always `.`, independent of UI language. Nonzero trailing zeros are retained, but `"0.00"` normalizes to initialized scale-zero zero. Invalid syntax is `InvalidFormat`.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
