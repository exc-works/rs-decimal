---
locale: de
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
translation_provenance: en-US-german-reviewed
api_id: Decimal::precision
---
# Decimal::precision

Die gespeicherte Anzahl der Nachkommastellen lesen.

```rust
fn precision (& self) -> isize
```

## Parameter, Rückgabe und Hinweise

Liefert isize (eine ganzzahlige number im Browser). Dies ist die Skala, nicht die Anzahl signifikanter Ziffern. Nachgestellte Nullen zählen mit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
