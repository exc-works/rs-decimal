---
locale: de
source_revision: dc01349bb451c857128d1cca5bf8b130f9b144b0e48f2d3acb755c072d9a7962
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo
---
# Decimal::quo

Dezimalzahlen mit expliziter oder aus den Eingaben abgeleiteter Genauigkeit dividieren.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Die Ausgabeskala ist die größere Operandenskala. Zwei Ganzzahleingaben ergeben daher eine Ganzzahl: 1 / 3 liefert mit Down den Wert 0, mit Up dagegen 1. Verwenden Sie quo_with_prec für Nachkommastellen. Ein Nulldivisor liefert DivideByZero. mode steuert die Rundung; Unnecessary liefert bei unexaktem Ergebnis RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
