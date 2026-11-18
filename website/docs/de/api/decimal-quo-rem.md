---
locale: de
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

Einen ganzzahligen Quotienten und einen vorzeichenbehafteten Decimal-Rest zurückgeben.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<(Decimal, Decimal), DecimalError>, abgebildet auf [quotient, remainder]. Der Quotient wird Richtung null auf Skala null abgeschnitten. Der Rest hat das Vorzeichen des Dividenden und die größere Operandenskala; self = quotient * other + remainder gilt. Jedes Ergebnis ist ein unabhängiger initialisierter Wrapper; Eingaben bleiben nutzbar. Ein Nulldivisor liefert DivideByZero vor der Skalenausrichtung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
