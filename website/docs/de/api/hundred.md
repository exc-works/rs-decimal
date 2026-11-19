---
locale: de
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
translation_provenance: en-US-german-reviewed
api_id: hundred
---
# hundred

Die Dezimalganzzahl einhundert konstruieren.

```rust
fn hundred () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen initialisierten Decimal mit Skala null. Dies ist ein Wertkonstruktor, keine gemeinsam veränderliche Konstante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
