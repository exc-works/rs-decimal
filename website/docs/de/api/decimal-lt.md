---
locale: de
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
translation_provenance: en-US-german-reviewed
api_id: Decimal::lt
---
# Decimal::lt

Prüfen, ob dieser Wert kleiner als ein anderer ist.

```rust
fn lt (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Numerisch gleiche Werte liefern false, auch bei unterschiedlichen gespeicherten Skalen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
