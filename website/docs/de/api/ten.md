---
locale: de
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
translation_provenance: en-US-german-reviewed
api_id: ten
---
# ten

Die Dezimalganzzahl zehn konstruieren.

```rust
fn ten () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen initialisierten Decimal mit Skala null. Verwenden Sie rescale, wenn Nachkommastellen benötigt werden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
