---
locale: de
source_revision: 8eec4d40194be2cd158d623391729d3b1e9d3b0532af669f3499b5168782b68c
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

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
