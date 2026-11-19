---
locale: de
source_revision: 94e5c200ef4c48b600cdc1b46dd606d6ca8e9e2741aedbb2bac50f53ac53727e
translation_provenance: en-US-german-reviewed
api_id: Decimal::new
---
# Decimal::new

Eine initialisierte Ganzzahl erzeugen. Verwenden Sie diesen Konstruktor für vorzeichenbehaftete 64-Bit-Ganzzahlen, etwa Anzahlen oder ganzzahlige Beträge.

```rust
fn new (value : i64) -> Self
```

## Parameter und Rückgabe

`value: i64` → JavaScript `bigint`. Das Ergebnis hat Skala null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="de" />

## Hinweise zur Verwendung

Verwenden Sie `12n`, keine JavaScript number. Werte außerhalb von i64 weist die JavaScript-API ab; Dezimalbrüche gehören in `new_from_string`.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
