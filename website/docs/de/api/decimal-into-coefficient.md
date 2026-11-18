---
locale: de
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
translation_provenance: en-US-german-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Den unskalierten Ganzzahlkoeffizienten herausbewegen.

```rust
fn into_coefficient (self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert BigInt in Rust und bigint in JS. Skala und Initialisierung gehen verloren: 1.2300 ergibt 12300. Die Standardnull ergibt 0. Der Decimal wird konsumiert und kann nicht erneut gelesen werden; coefficient liefert in der Bridge einen nicht konsumierenden Snapshot.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
