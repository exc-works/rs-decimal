---
locale: de
source_revision: 5a350c30c073c772c4921158a8a0d829d4480ae375ec35f7532a213f3f45e557
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

Den Zahlenwert einer vorzeichenbehafteten Ganzzahl beibehalten und zusätzliche Nachkommastellen speichern. Nützlich, wenn ein ganzer Betrag als 12.00 dargestellt werden soll.

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`value: i64` → `bigint`; `prec: isize` → ganzzahlige `number`. Der gespeicherte Koeffizient wird `value × 10^prec`, die Skala `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="de" />

## Hinweise zur Verwendung

`string()` gibt 12 aus; `string_with_trailing_zeros()` gibt 12.00 aus. Anders als bei `new_with_prec` wird der Zahlenwert nicht zu 0.12. Negative Präzision liefert `InvalidPrecision`; große Anforderungen werden vor dem Anlegen von Zehnerpotenzen abgewiesen.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
