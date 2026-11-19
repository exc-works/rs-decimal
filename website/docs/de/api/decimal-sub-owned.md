---
locale: de
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
translation_provenance: en-US-german-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

Den linken Operanden für exakte Arithmetik konsumieren.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen initialisierten Decimal mit der größeren Eingabeskala. Der Empfänger wird verschoben, other bleibt nutzbar. Speicher kann wiederverwendet werden, Wachstum kann aber Allokationen erfordern. Die Bridge weist denselben Wrapper als Empfänger und other vor dem Move ab. Nach Eintritt in Rust konsumiert selbst ein Ressourcenfehler den Empfänger.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
