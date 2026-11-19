---
locale: de
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
translation_provenance: en-US-german-reviewed
api_id: Decimal::gt
---
# Decimal::gt

Prüfen, ob dieser Wert größer als ein anderer ist.

```rust
fn gt (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Numerisch gleiche Werte liefern false, auch bei unterschiedlichen gespeicherten Skalen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
