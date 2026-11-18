---
locale: de
source_revision: 6f40656db82d99e820bf25f176794a430859d18b708e5dfb0e4c5ad3ab2117e5
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

Prüfen, ob der Zahlenwert keinen Nachkommaanteil besitzt, also ganzzahlig ist.

```rust
fn is_integer (& self) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert bool/boolean. Geprüft wird der Zahlenwert, nicht eine Skala von null: 12.00 ist eine Ganzzahl. Die Standardnull ist ebenfalls ganzzahlig und hat keinen Bruchteil. Diese Abfragen initialisieren oder verändern die Eingabe nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
