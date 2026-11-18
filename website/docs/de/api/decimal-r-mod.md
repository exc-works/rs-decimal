---
locale: de
source_revision: 93dfb354f7906fd9c23e3b9ad99dace81c54c1bb0757216e3e8aa83d274f8bb9
translation_provenance: en-US-german-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

Den vorzeichenbehafteten Rest einer Richtung null abgeschnittenen Division zurückgeben.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Der Rest hat das Vorzeichen des Dividenden und die größere Operandenskala; dies ist kein nichtnegativer euklidischer Modulo. Ein Nulldivisor liefert DivideByZero. Eingaben werden ausgeliehen. Rust verlangt r#mod, da mod ein Schlüsselwort ist; JS verwendet .mod(other). Beide Namen rufen dasselbe Kernverhalten auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="de" />

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
