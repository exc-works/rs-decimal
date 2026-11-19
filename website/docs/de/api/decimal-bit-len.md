---
locale: de
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
translation_provenance: en-US-german-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

Bits des absoluten unskalierten Koeffizienten zählen.

```rust
fn bit_len (& self) -> usize
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert usize, die JavaScript-API number. Die Skala wird nicht angewandt und das Vorzeichenbit nicht gezählt: -1.20 hat Koeffizient -120 und Bitlänge 7. Null einschließlich Standardnull liefert 0. Dies ist nicht der Index des höchsten gesetzten Bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
