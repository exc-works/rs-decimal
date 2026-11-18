---
locale: de
source_revision: dfbad807bf85fc60fbbfb720800bbf36eb483a86897e4245a66f95ff012fb636
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

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
