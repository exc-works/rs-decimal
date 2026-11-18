---
locale: de
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
translation_provenance: en-US-german-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

Nichtnegativität verlangen und eine initialisierte Kopie zurückgeben.

```rust
fn must_non_negative (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert für nichtnegative Eingaben einschließlich null einen Decimal. Eine negative Eingabe löst in Rust einen Panic mit Zeichenfolgen-Payload aus. In Wasm ist dies ein Trap, kein behandelbarer DecimalError; der Playground-Lauf endet. Ein neuer Lauf erzeugt frischen Wasm-Zustand. Dies ist eine numerische Zusicherung, keine Absolutwertkonvertierung; verwenden Sie abs zum Ändern des Vorzeichens.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
