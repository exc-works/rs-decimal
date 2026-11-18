---
locale: de
source_revision: 2f8f701187893878a876b218458ba92b8464c997083c08e62c584ef14b118454
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

## Hinweise und Grenzen

Negative Werte und Werte außerhalb des Bereichs sind Bridge-Parameterfehler. Negative Präzision liefert `InvalidPrecision`; große Präzision erreicht die separate Ressourcengrenze des Playgrounds. Die Ausgabe mit nachgestellten Nullen zeigt die angehängten Stellen.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
