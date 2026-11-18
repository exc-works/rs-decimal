---
locale: de
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
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

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
