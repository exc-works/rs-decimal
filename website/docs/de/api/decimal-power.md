---
locale: de
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
translation_provenance: en-US-german-reviewed
api_id: Decimal::power
---
# Decimal::power

Bei ursprünglicher Skala mit einem ganzzahligen Exponenten potenzieren.

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

power: i64, als bigint übertragen. Rundet HalfEven auf die Eingabeskala; negative Exponenten bilden den Kehrwert. 2.00 hoch -3 ergibt 0.12, ganzzahlig skaliertes 2 hoch -3 dagegen 0. Null hoch null ist initialisierte eins; null hoch einem negativen Exponenten liefert DivideByZero. Riesige Exponenten über der Kerngrenze für Zwischenwerte liefern Overflow vor der Allokation. Der Browser hat kleinere Skalen-/Koeffizientenbudgets für Zwischenwerte; 0 und +/-1 behalten die schnellen Kernpfade. Die Eingabe wird ausgeliehen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
