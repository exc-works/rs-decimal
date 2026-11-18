---
locale: de
source_revision: fe9121bf755dd82c88fc1274dc43b09d6173d99c7a22441eb84bc6a73587fa71
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

## Hinweise und Grenzen

Verwenden Sie `12n`, keine JavaScript number. Werte außerhalb von i64 weist die Bridge ab; Dezimalbrüche gehören in `new_from_string`.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
