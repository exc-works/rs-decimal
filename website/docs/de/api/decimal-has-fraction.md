---
locale: de
source_revision: 4f94d3a29039b5504e83148308232156a8a140f150fc1e38d0956e8fb963c92b
translation_provenance: en-US-german-reviewed
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Prüfen, ob der Zahlenwert einen Nachkommaanteil besitzt.

```rust
fn has_fraction (& self) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert bool/boolean. Geprüft wird der Zahlenwert, nicht eine Skala von null: 12.00 ist eine Ganzzahl. Die Standardnull ist ebenfalls ganzzahlig und hat keinen Bruchteil. Diese Abfragen initialisieren oder verändern die Eingabe nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
