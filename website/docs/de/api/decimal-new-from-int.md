---
locale: de
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
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

## Hinweise und Grenzen

Das Browserziel ist wasm32; der akzeptierte Bereich reicht deshalb von -2147483648 bis 2147483647, auch wenn natives Rust auf einem 64-Bit-System läuft. Bruchzahlen, NaN und Unendlichkeit sind Parameterfehler.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
