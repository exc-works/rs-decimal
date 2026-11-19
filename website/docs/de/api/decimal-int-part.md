---
locale: de
source_revision: 0b7e4ede82a7865b24f3ea40debe791505bf9ac255c8e2f8790aa1271d0fdf3a
translation_provenance: en-US-german-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Den Ganzzahlteil durch Abschneiden in Richtung null extrahieren.

```rust
fn int_part (& self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert BigInt, die JavaScript-API bigint, ohne Gleitkommakonvertierung. -12.75 ergibt -12, nicht floor(-12.75) = -13. Die Standardnull ergibt null. Das Beispiel serialisiert die Ganzzahl als Zeichenfolge, um beliebige Genauigkeit in JSON zu erhalten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
