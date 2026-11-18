---
locale: de
source_revision: 6c8f91b0409cc71714a4b9926acf34a3cf0aa635f8904f2e607ed3243fb8b30a
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

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
