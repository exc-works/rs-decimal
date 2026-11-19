---
locale: de
source_revision: dcbc410f5dabf4a62b4cb667b765d034201be05823aeeb413148caad325fb23f
translation_provenance: en-US-german-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

Den exakten unskalierten Ganzzahlkoeffizienten kopieren.

```rust
fn big_int (& self) -> BigInt
```

## Parameter, Rückgabe und Hinweise

Rust liefert einen unabhängigen BigInt, die Bridge bigint. Anders als eine abschneidende Ganzzahlkonvertierung enthält dieser nachgestellte Nullen. Das Beispiel stellt bigint für JSON als Text dar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
