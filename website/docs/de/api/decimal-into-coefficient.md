---
locale: de
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
translation_provenance: en-US-german-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Den unskalierten Ganzzahlkoeffizienten herausbewegen.

```rust
fn into_coefficient (self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert BigInt in Rust und bigint in JS. Skala und Initialisierung gehen verloren: 1.2300 ergibt 12300. Die Standardnull ergibt 0. Der Decimal wird konsumiert und kann nicht erneut gelesen werden; coefficient liefert in der Bridge einen nicht konsumierenden Snapshot.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
