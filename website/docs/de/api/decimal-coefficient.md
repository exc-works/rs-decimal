---
locale: de
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
translation_provenance: en-US-german-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

Den exakten unskalierten Ganzzahlkoeffizienten untersuchen.

```rust
fn coefficient (& self) -> & BigInt
```

## Parameter, Rückgabe und Hinweise

Rust leiht &BigInt aus; die JavaScript-API liefert einen unveränderlichen bigint-Snapshot, keine live aktualisierte Referenz. Der Koeffizient enthält nachgestellte Nullen und ist nicht der Ganzzahlteil der Dezimalzahl. Das Beispiel gibt bigint als Text aus, damit JSON alle Ziffern erhält.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
