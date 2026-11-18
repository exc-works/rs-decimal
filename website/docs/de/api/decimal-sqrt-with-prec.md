---
locale: de
source_revision: 9142df8b7aad31accc1292d48cdb5af145b94888294db67afaa241b5146ec0d5
translation_provenance: en-US-german-reviewed
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

Eine Quadratwurzel berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. Kein Gradparameter. Negative Eingabe liefert NegativeRoot. prec: isize, als ganzzahlige number übertragen, wählt Nachkommastellen. Negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt die Ausgabeskala auf 1000. Nichttriviale Wurzeln haben höchstens Grad 128; Skalierung, Newton-Potenzen und Zwischenwerte der Mittelpunktzertifizierung werden vorgeprüft. Null, vorzeichenbehaftete Einheitswerte und Wurzeln ersten Grades behalten schnelle Kernpfade, unterliegen aber weiter den Ausgabepräzisionsgrenzen. Definitionsbereichsfehler und negative Präzision behalten Bibliotheksverhalten; Ressourcenfehler unterscheiden sich von NoConvergence. Gültige Einzeleingaben garantieren nicht, dass jede Grad-/Präzisionskombination ins Browserbudget passt.

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
