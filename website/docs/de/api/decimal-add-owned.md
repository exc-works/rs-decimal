---
locale: de
source_revision: 181ceff066af430ef0b4f061e0d4890d14b967a20ab1d2fe96ac03e3b4f024fe
translation_provenance: en-US-german-reviewed
api_id: Decimal::add_owned
---
# Decimal::add_owned

Den linken Operanden für exakte Arithmetik konsumieren.

```rust
fn add_owned (self , other : & Self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen initialisierten Decimal mit der größeren Eingabeskala. Der Empfänger wird verschoben, other bleibt nutzbar. Speicher kann wiederverwendet werden, Wachstum kann aber Allokationen erfordern. Die JavaScript-API weist denselben Wrapper als Empfänger und other vor dem Move ab. Nach Eintritt in Rust konsumiert selbst ein Ressourcenfehler den Empfänger.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
