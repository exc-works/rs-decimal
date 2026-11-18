---
locale: de
source_revision: afcf4f720c241e6c6d4a096ef63464bb4e6b4cfb060705652f6b168cfc059cbe
translation_provenance: en-US-german-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

Dezimalzahlen exakt addieren und ein nichtnegatives Ergebnis verlangen.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern. Rust liefert Result: Nur das Ergebnis muss nichtnegativ sein, nicht jeder Operand. Ein negatives Ergebnis liefert InvalidArgument; null ist erlaubt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
