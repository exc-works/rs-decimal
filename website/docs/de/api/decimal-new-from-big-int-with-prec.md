---
locale: de
source_revision: 7a4da4f8ddb22857191bcc95e49dde5161555ab6aaf013e72ff12ef5cff5913d
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

Einen beliebig großen Koeffizienten kopieren und seine Nachkommastellen festlegen. Auch große Ganzzahleingaben bleiben exakt.

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: &BigInt` → `bigint`; `prec: isize` → ganzzahlige `number`. Liefert `value × 10^-prec` und leiht die Rust-Eingabe aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="de" />

## Hinweise zur Verwendung

Dies ändert die Interpretation des Koeffizienten, nicht seine Ziffern. Negative Präzision liefert `InvalidPrecision`. Der Playground begrenzt Eingabeziffern und Skala unabhängig; `resource`-Fehler bedeuten keinen Verlust numerischer Genauigkeit.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
