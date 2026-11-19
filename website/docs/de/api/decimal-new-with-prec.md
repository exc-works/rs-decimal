---
locale: de
source_revision: 2cba3a645b36c53756ae013b43e89956dee5ee0f736cbeccecbeb94e8966c7b4
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

Eine vorzeichenbehaftete Ganzzahl als Koeffizienten bei einer Nachkommastellenzahl interpretieren. Beispielsweise bedeutet 12 mit Präzision 2 den Wert 0.12.

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: i64` → `bigint`; `prec: isize` → ganzzahlige `number`. Liefert `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="de" />

## Hinweise zur Verwendung

Präzision zählt Nachkommastellen, nicht signifikante Ziffern. Dies hängt keine Nullen an eine numerisch unveränderte Ganzzahl an; verwenden Sie für 12.00 `new_with_append_prec`. Negative Präzision liefert `InvalidPrecision`. Ein Nullkoeffizient behält die angeforderte Skala.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
