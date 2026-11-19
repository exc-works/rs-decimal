---
locale: de
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
translation_provenance: en-US-german-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

Zwei Zahlen vergleichen und eines von drei Ergebnissen erhalten.

```rust
fn cmp (& self , other : & Self) -> i32
```

## Parameter, Rückgabe und Hinweise

other: &Decimal (Decimal in JavaScript) ausleihen. Liefert i32/number: -1, 0 oder 1. Skala und Initialisierung beeinflussen die numerische Gleichheit nicht. Dies ist die inhärente Methode, nicht Ord::cmp mit dem Rückgabetyp Ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
