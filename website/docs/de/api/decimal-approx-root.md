---
locale: de
source_revision: 2e472bd0598b8ec796fb5adb09d98ce669a8d1defb11e5244f124c816e8d2cd8
translation_provenance: en-US-german-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

Eine Wurzel positiven Grades berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. degree: i64, als bigint übertragen; muss positiv sein, sonst InvalidRoot. Ungerade Grade erlauben negative Eingaben; gerade Wurzeln negativer Werte liefern NegativeRoot. Grad eins entspricht HalfEven-Reskalierung. Die Standardpräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="de" />

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
