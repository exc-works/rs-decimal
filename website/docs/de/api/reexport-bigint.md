---
locale: de
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
translation_provenance: en-US-german-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

BigInt aus der Crate-Wurzel importieren.

```rust
pub use num_bigint::BigInt as BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Dies ist der öffentliche Rust-Reexport, kein neuer Typ und keine neue Implementierung. Die Importsyntax wird nativ getestet. Browserentsprechungen sind bigint, ein exaktes rationales Feldobjekt bzw. die globale Funktion most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
