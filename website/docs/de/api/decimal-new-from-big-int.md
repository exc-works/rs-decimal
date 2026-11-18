---
locale: de
source_revision: e8f2b09a741088cc99921865f54ef434d0d70ac376d4e29b7efc3c00a51c3956
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

## Hinweise und Grenzen

Wandeln Sie eine JavaScript number nicht erst nach einem Genauigkeitsverlust in bigint um. Beginnen Sie mit einem auf `n` endenden Ganzzahlliteral oder exaktem Ganzzahltext. Der Playground akzeptiert höchstens 4096 Eingabeziffern; diese Playground-Regel gilt nicht für die native Bibliothek.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
