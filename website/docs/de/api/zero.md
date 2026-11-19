---
locale: de
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
translation_provenance: en-US-german-reviewed
api_id: zero
---
# zero

Eine initialisierte Null mit Skala null konstruieren.

```rust
fn zero () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen Decimal. Anders als Decimal::default() ist das Ergebnis initialisiert, obwohl beide numerisch gleich sind.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
