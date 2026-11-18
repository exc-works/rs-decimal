---
locale: de
source_revision: 49e05880780d4157422be53a7d3d5c1a0d8d4e809a77bd78ae251e8f6c7889c8
translation_provenance: en-US-german-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

Eine Quadratwurzel berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. Kein Gradparameter. Negative Eingabe liefert NegativeRoot. Die Standardpräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt die Ausgabeskala auf 1000. Nichttriviale Wurzeln haben höchstens Grad 128; Skalierung, Newton-Potenzen und Zwischenwerte der Mittelpunktzertifizierung werden vorgeprüft. Null, vorzeichenbehaftete Einheitswerte und Wurzeln ersten Grades behalten schnelle Kernpfade, unterliegen aber weiter den Ausgabepräzisionsgrenzen. Definitionsbereichsfehler und negative Präzision behalten Bibliotheksverhalten; Ressourcenfehler unterscheiden sich von NoConvergence. Gültige Einzeleingaben garantieren nicht, dass jede Grad-/Präzisionskombination ins Browserbudget passt.

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
