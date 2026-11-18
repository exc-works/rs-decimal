---
locale: de
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
translation_provenance: en-US-german-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

Eine Dezimalzahl beim Ändern ihrer Nachkommastellen konsumieren.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize; mode: RoundingMode. Wie rescale rundet dies oder hängt Nullen an und initialisiert das Ergebnis stets. Der Empfänger wird auch bei InvalidPrecision, RoundUnnecessary oder einem Browser-Ressourcenfehler aus Rust verschoben. Ungültige JS-Typen/Bereiche werden vor Rust abgewiesen und konsumieren ihn nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
