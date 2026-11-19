---
locale: de
source_revision: f7f692709bde775f34d7f5feb756c59944ff77c763bae1798b2b4306d9770c41
translation_provenance: en-US-german-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

Den exakten unskalierten Ganzzahlkoeffizienten kopieren.

```rust
fn big_int (& self) -> BigInt
```

## Parameter, Rückgabe und Hinweise

Rust liefert einen unabhängigen BigInt, die JavaScript-API bigint. Anders als eine abschneidende Ganzzahlkonvertierung enthält dieser nachgestellte Nullen. Das Beispiel stellt bigint für JSON als Text dar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
