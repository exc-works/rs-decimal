---
locale: de
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
translation_provenance: en-US-german-reviewed
api_id: between
---
# between

Prüfen, ob eine Dezimalzahl innerhalb eines einschließlich seiner Grenzen geltenden Zahlenbereichs liegt.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## Parameter, Rückgabe und Hinweise

v, lower und upper als &Decimal ausleihen; liefert bool/boolean für lower <= v <= upper. Gleichheit ignoriert die Darstellung. Grenzen werden nicht umgeordnet: lower > upper liefert false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
