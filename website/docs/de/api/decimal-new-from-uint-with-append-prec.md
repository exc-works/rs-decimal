---
locale: de
source_revision: 1f2a665c28ead224f75ed902dd02b6b36626040f13db084f643088b5ff82188f
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

Den Zahlenwert einer vorzeichenlosen Ganzzahl beibehalten und gespeicherte Nachkommastellen anhängen.

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: u64` → nichtnegatives `bigint`; `prec: isize` → ganzzahlige `number`. Das Ergebnis erhält den Ganzzahlwert mit Skala `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="de" />

## Hinweise zur Verwendung

Negative Werte und Werte außerhalb des Bereichs sind Bridge-Parameterfehler. Negative Präzision liefert `InvalidPrecision`. Die Ausgabe mit nachgestellten Nullen zeigt die angehängten Stellen.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
