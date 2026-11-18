---
locale: de
source_revision: 4d2afeddcd523e94b9e9276fe6f8c19193bfe95254c1ef9ded67b301a566152b
translation_provenance: en-US-german-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

Durch numerischen Vergleich einen ursprünglichen Operanden auswählen.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert eine gemeinsame Referenz mit der Lebensdauer der Eingabe. Bei Gleichheit wird self ausgewählt, unter Erhalt von Skala und Initialisierung. Die Bridge liefert den entsprechenden ursprünglichen Wrapper, keine Kopie. Verschieben/Freigeben macht alle JS-Aliase darauf ungültig; JS erzwingt keine Rust-Borrow-Lebensdauern. max/min liefern ein unabhängiges Ergebnis mit eigenem Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
