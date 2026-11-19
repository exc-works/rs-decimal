---
locale: en-US
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
api_id: ten
---
# ten

Construct the decimal integer ten.

```rust
fn ten () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. Use rescale when decimal places are required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="en-US" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
