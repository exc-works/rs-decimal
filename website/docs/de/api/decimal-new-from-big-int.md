---
locale: de
source_revision: 41da382d7882f55308ed631f052fb5ac78093f335752392d35d023f744890035
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

Eine beliebig große Ganzzahl ohne Ziffernverlust in einen initialisierten Decimal mit Skala null kopieren.

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## Parameter und Rückgabe

Rust leiht `&BigInt` aus; JavaScript übergibt unveränderliches `bigint`. Der zurückgegebene Koeffizient ist unabhängig von der Quelle.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="de" />

## Hinweise zur Verwendung

Wandeln Sie eine JavaScript number nicht erst nach einem Genauigkeitsverlust in bigint um. Beginnen Sie mit einem auf `n` endenden Ganzzahlliteral oder exaktem Ganzzahltext.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
