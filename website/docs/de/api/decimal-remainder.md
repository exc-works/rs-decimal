---
locale: de
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
translation_provenance: en-US-german-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

Ganzzahlteil und unskalierten Nachkommarest trennen.

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (BigInt, BigInt), abgebildet auf [bigint, bigint]. Der erste Wert wird Richtung null abgeschnitten; der zweite ist ein vorzeichenbehafteter Koeffizientenrest, kein Decimal-Bruchteil. Für -12.750 entsteht (-12, -750): Der zweite Wert benötigt die ursprüngliche Skala 3, um -0.750 zu bedeuten. Die Standardnull liefert (0, 0).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
