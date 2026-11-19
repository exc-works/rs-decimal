---
locale: de
source_revision: f67e1caa0fb769c278b4d8058441646a0cf632963d9da4fd8b19596b64289dc8
translation_provenance: en-US-german-reviewed
api_id: reexport::BigRational
---
# reexport::BigRational

BigRational aus der Crate-Wurzel importieren.

```rust
pub use num_rational::BigRational as BigRational
```

## Parameter, Rückgabe und übliche Verwendung

Dies ist der öffentliche Rust-Reexport, kein neuer Typ und keine neue Implementierung. Die Importsyntax wird nativ getestet. Browserentsprechungen sind bigint, ein exaktes rationales Feldobjekt bzw. die globale Funktion most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigrational" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
