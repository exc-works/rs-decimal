---
locale: de
source_revision: 4842619da75eb5fbcf11cdce1b3ca781ba18031962688508c5f82ef39efdcf01
translation_provenance: en-US-german-reviewed
api_id: Decimal::quo
---
# Decimal::quo

Dezimalzahlen mit expliziter oder aus den Eingaben abgeleiteter Genauigkeit dividieren.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert Result<Decimal, DecimalError>. Die Ausgabeskala ist die größere Operandenskala. Zwei Ganzzahleingaben ergeben daher eine Ganzzahl: 1 / 3 liefert 0. Verwenden Sie quo_with_prec für Nachkommastellen. Ein Nulldivisor liefert DivideByZero. mode steuert die Rundung; Unnecessary liefert bei unexaktem Ergebnis RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
