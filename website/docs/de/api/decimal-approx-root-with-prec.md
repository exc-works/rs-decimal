---
locale: de
source_revision: 3553acc8fce2edd77d63d567220e36aee7b664b18f15fee50a17c4ee358a3283
translation_provenance: en-US-german-reviewed
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

Eine Wurzel positiven Grades berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. degree: i64, als bigint übertragen; muss positiv sein, sonst InvalidRoot. Ungerade Grade erlauben negative Eingaben; gerade Wurzeln negativer Werte liefern NegativeRoot. Grad eins entspricht HalfEven-Reskalierung. prec: isize, als ganzzahlige number übertragen, wählt Nachkommastellen. Negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt die Ausgabeskala auf 1000. Nichttriviale Wurzeln haben höchstens Grad 128; Skalierung, Newton-Potenzen und Zwischenwerte der Mittelpunktzertifizierung werden vorgeprüft. Null, vorzeichenbehaftete Einheitswerte und Wurzeln ersten Grades behalten schnelle Kernpfade, unterliegen aber weiter den Ausgabepräzisionsgrenzen. Definitionsbereichsfehler und negative Präzision behalten Bibliotheksverhalten; Ressourcenfehler unterscheiden sich von NoConvergence. Gültige Einzeleingaben garantieren nicht, dass jede Grad-/Präzisionskombination ins Browserbudget passt.

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
