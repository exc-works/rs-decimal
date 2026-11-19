---
locale: de
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: en-US-german-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Den Ganzzahlteil durch Abschneiden in Richtung null extrahieren.

```rust
fn int_part (& self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert BigInt, die Bridge bigint, ohne Gleitkommakonvertierung. -12.75 ergibt -12, nicht floor(-12.75) = -13. Die Standardnull ergibt null. Das Beispiel serialisiert die Ganzzahl als Zeichenfolge, um beliebige Genauigkeit in JSON zu erhalten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
