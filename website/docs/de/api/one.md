---
locale: de
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
translation_provenance: en-US-german-reviewed
api_id: one
---
# one

Die Dezimalganzzahl eins konstruieren.

```rust
fn one () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen initialisierten Decimal mit Skala null. Ändern oder Freigeben eines Ergebnisses beeinflusst spätere Aufrufe nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
