---
locale: de
source_revision: d578df6b6b71b8ce4b0e165f6e630705dc946bcefab55f8b43fc9cba6d60bb70
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

Verwenden Sie `12n`, keine JavaScript number. Werte außerhalb von i64 weist die Bridge ab; Dezimalbrüche gehören in `new_from_string`.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
