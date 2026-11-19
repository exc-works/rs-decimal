---
locale: de
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
translation_provenance: en-US-german-reviewed
api_id: most_significant_bit
---
# most_significant_bit

Den nullbasierten Index des höchsten gesetzten Bits ermitteln.

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigInt, dargestellt als bigint. Liefert Result<usize, DecimalError>/number. Null und eins liefern beide 0; 120 liefert 6. Negative Eingabe liefert InvalidArgument. Dies ist eine freie Funktion, keine Decimal-Methode; übergeben Sie coefficient(), wenn dessen Ganzzahl gemeint ist.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
