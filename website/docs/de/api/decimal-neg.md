---
locale: de
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
translation_provenance: en-US-german-reviewed
api_id: Decimal::neg
---
# Decimal::neg

Eine unabhängige Dezimalzahl mit umgekehrtem Vorzeichen erzeugen.

```rust
fn neg (& self) -> Self
```

## Parameter, Rückgabe und Hinweise

Liefert Decimal und erhält Skala und nachgestellte Nullen. Null bleibt numerisch null; auch bei Standardeingabe ist das Ergebnis initialisiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
