---
locale: de
source_revision: b1a9d67b6db859834f3ef9cd2d0fefcd4239bbf10472aedba8e0a86a95b51eeb
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

Negative Werte und Werte außerhalb des Bereichs sind JavaScript-Parameterfehler. Negative Präzision liefert `InvalidPrecision`. Die Ausgabe mit nachgestellten Nullen zeigt die angehängten Stellen.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
