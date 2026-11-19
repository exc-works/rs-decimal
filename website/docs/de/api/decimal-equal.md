---
locale: de
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
translation_provenance: en-US-german-reviewed
api_id: Decimal::equal
---
# Decimal::equal

Prüfen, ob zwei Dezimalzahlen denselben Zahlenwert darstellen.

```rust
fn equal (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Skala und Initialisierung werden ignoriert. JavaScript === vergleicht die Identität der Wrapper; equal() prüft die numerische Gleichheit von Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
