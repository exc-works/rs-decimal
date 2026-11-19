---
locale: de
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: en-US-german-reviewed
api_id: Decimal::sign
---
# Decimal::sign

Das Vorzeichen als -1, 0 oder 1 lesen.

```rust
fn sign (& self) -> i32
```

## Parameter, Rückgabe und Hinweise

Liefert i32/number. Initialisierte und nicht initialisierte Null haben Vorzeichen 0; es gibt kein separates negatives Nullvorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
