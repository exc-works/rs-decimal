---
locale: de
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
translation_provenance: en-US-german-reviewed
api_id: Decimal::scale
---
# Decimal::scale

Die gespeicherte nichtnegative Dezimalskala lesen.

```rust
fn scale (& self) -> usize
```

## Parameter, Rückgabe und Hinweise

Liefert usize als exakte number innerhalb des Skalenbudgets des Playgrounds. Der Wert entspricht precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
