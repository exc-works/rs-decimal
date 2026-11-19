---
locale: de
source_revision: 0bbd6c78da40405c6e9ad9c68a14878f98d98b4601eb8ff1296abba0bbf809ae
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

## Verwandte Methoden

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
