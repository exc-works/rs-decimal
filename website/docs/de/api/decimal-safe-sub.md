---
locale: de
source_revision: 8969e894369c10d2e27ead76f07916d45a77105ffad604c0d4f46fcf59b84751
translation_provenance: en-US-german-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

Dezimalzahlen exakt subtrahieren und ein nichtnegatives Ergebnis verlangen.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern. Rust liefert Result: Nur das Ergebnis muss nichtnegativ sein, nicht jeder Operand. Ein negatives Ergebnis liefert InvalidArgument; null ist erlaubt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
