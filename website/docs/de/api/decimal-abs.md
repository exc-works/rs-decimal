---
locale: de
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
translation_provenance: en-US-german-reviewed
api_id: Decimal::abs
---
# Decimal::abs

Eine unabhängige Dezimalzahl mit dem Absolutwert erzeugen.

```rust
fn abs (& self) -> Self
```

## Parameter, Rückgabe und Hinweise

Liefert Decimal und erhält Skala und nachgestellte Nullen. Das Ergebnis ist auch bei Standardeingabe initialisiert; die Eingabe bleibt unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
