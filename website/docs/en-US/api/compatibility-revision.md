---
locale: en-US
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

Inspect the upstream Go revision used for compatibility checks.

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## Normal usage and semantics

This immutable &str is a source revision, not the current Rust/web build commit. The browser reads the same constant from compiled Rust. Do not interpret it as a package version or deployment identifier.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="en-US" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
