---
locale: de
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
translation_provenance: en-US-german-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

Den vorzeichenbehafteten Rest einer Richtung null abgeschnittenen Division zurückgeben.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Der Rest hat das Vorzeichen des Dividenden und die größere Operandenskala; dies ist kein nichtnegativer euklidischer Modulo. Ein Nulldivisor liefert DivideByZero. Eingaben werden ausgeliehen. Rust verlangt r#mod, da mod ein Schlüsselwort ist; JS verwendet .mod(other). Beide Namen rufen dasselbe Kernverhalten auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
