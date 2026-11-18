---
locale: de
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

Dezimalzahlen exakt multiplizieren.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala ist die Summe der Operandenskalen; jede Produktziffer bleibt erhalten. Skalenüberlauf im Kern liefert Overflow. Es gibt keinen Rundungsmodusparameter.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
