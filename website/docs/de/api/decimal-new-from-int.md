---
locale: de
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

Aus einer vorzeichenbehafteten Ganzzahl nativer Breite eine initialisierte Ganzzahl erzeugen.

```rust
fn new_from_int (value : isize) -> Self
```

## Parameter und Rückgabe

`value: isize` → ganzzahlige JavaScript `number`; die Ergebnisskala ist null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="de" />

## Hinweise zur Verwendung

Das Browserziel ist wasm32; der akzeptierte Bereich reicht deshalb von -2147483648 bis 2147483647, auch wenn natives Rust auf einem 64-Bit-System läuft. Bruchzahlen, NaN und Unendlichkeit sind Parameterfehler.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
