---
locale: de
source_revision: 53a165ed47764f5f661387617a1b68ba2abdefa181cac1cde270d551da87797c
translation_provenance: en-US-german-reviewed
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

Eine Quadratwurzel berechnen. Kein Gradparameter. Negative Eingabe liefert NegativeRoot. prec: isize, als ganzzahlige number übertragen, wählt Nachkommastellen. Negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="de" />

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
