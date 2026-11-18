---
locale: de
source_revision: ce941de7d4ef355a538d2c2c33a609ab4eeb3b4645eb801e81a24755fcf0ca5a
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

Dezimalzahlen mit festgelegter Ausgabepräzision multiplizieren.

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala beträgt prec Nachkommastellen und muss nichtnegativ sein; negatives prec liefert InvalidPrecision. Ein expliziter mode steuert die Rundung; Unnecessary liefert RoundUnnecessary, wenn Ziffern ungleich null verworfen werden müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
