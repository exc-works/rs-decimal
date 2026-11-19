---
locale: de
source_revision: 7f9e2f6ed775898aae666efe4610ba891f54aadb6987e2a347de88c3c59500ea
translation_provenance: en-US-german-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

Eine Quadratwurzel berechnen. Kein Gradparameter. Negative Eingabe liefert NegativeRoot. Die Standardpräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Gerundet wird HalfEven; exakte Mittelpunktswurzeln wie sqrt(6.25) bei Präzision null werden auf 2 gerundet. Trotz des Namens approx_root zertifiziert Rust die Rundung mit Ganzzahlvergleichen; erschöpfte Rechen-/Iterationsbudgets des Kerns liefern NoConvergence. Die Eingabe wird ausgeliehen und bleibt unverändert; Ergebnisse einschließlich Standardnull sind initialisiert.

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
