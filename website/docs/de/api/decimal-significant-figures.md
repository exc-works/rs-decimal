---
locale: de
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
translation_provenance: en-US-german-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

Auf höchstens die gewählte Anzahl signifikanter Dezimalziffern runden.

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

figures: positives isize/ganzzahlige number; mode wählt die Rundung. Anders als Nachkommastellen zählen signifikante Stellen Koeffizientenziffern. Dies erhöht niemals die Präzision und kann Ganzzahlziffern runden (1234.5 → 1230 bei drei Stellen). Nichtpositive figures liefern InvalidArgument; nötige Rundung mit Unnecessary liefert RoundUnnecessary. Ein möglicher Koeffizientenübertrag wird zuerst gegen das Browserbudget geprüft.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
