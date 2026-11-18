---
locale: de
source_revision: 7e14f36bf17f5e06d930d4aefd01137eff4589a325eb7eee0237e89f396a3ba8
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

## Hinweise und Grenzen

`string()` gibt 12 aus; `string_with_trailing_zeros()` gibt 12.00 aus. Anders als bei `new_with_prec` wird der Zahlenwert nicht zu 0.12. Negative Präzision liefert `InvalidPrecision`; große Anforderungen werden vor dem Anlegen von Zehnerpotenzen abgewiesen.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
