---
locale: en-US
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
api_id: Decimal::r#mod
---
# Decimal::r#mod

Return the signed remainder of truncated division.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. The remainder has the dividend sign and the larger operand scale; this is not a nonnegative Euclidean modulo. Zero divisor returns DivideByZero. Inputs are borrowed. Rust requires r#mod because mod is a keyword; JS uses .mod(other). Both names call the same core behavior.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
