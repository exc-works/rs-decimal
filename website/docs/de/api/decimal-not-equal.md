---
locale: de
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: en-US-german-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

Prüfen, ob zwei Dezimalzahlen verschiedene Zahlenwerte darstellen.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Dies ist die Umkehrung von equal(), kein Vergleich von Koeffizienten-/Skalensnapshots.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
