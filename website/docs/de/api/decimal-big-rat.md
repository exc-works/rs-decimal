---
locale: de
source_revision: 742bab219fea4dd88e3233aac7a28750ce01392539e76e100af497e171dcc4c9
translation_provenance: en-US-german-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

Den exakten gekürzten rationalen Wert lesen.

```rust
fn big_rat (& self) -> BigRational
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert BigRational. Die JavaScript-API liefert einen unveränderlichen Snapshot {numerator: bigint, denominator: bigint}, normalisiert mit positivem Nenner. Dezimalskala und nachgestellte Nullen bleiben nicht als rationale Metadaten erhalten; null wird 0/1. Die Eingabe wird ausgeliehen und bleibt unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
