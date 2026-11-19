---
locale: de
source_revision: cc52467a55bcaa7e86e52d9cf523d5b8a22924e126619052d59e51cd0b6aab8b
translation_provenance: en-US-german-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Den unskalierten Ganzzahlkoeffizienten herausbewegen.

```rust
fn into_coefficient (self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert BigInt in Rust und bigint in JS. Skala und Initialisierung gehen verloren: 1.2300 ergibt 12300. Die Standardnull ergibt 0. Der Decimal wird konsumiert und kann nicht erneut gelesen werden; coefficient liefert in JavaScript einen nicht konsumierenden Snapshot.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
