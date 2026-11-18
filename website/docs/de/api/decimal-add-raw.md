---
locale: de
source_revision: 872ef2a43e7ac5d06f4ea120672471b0a862ee6f8a70e48db9f1f9367f2518f7
translation_provenance: en-US-german-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

Den unskalierten Koeffizienten um eine i64-Ganzzahl ändern.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

value: i64, in der Bridge als bigint dargestellt. Die Skala bleibt erhalten: Bei Skala 2 ändert Addieren/Subtrahieren von 1 die Zahl um 0.01, nicht um 1. Rust liefert einen neuen Decimal. number und bigint außerhalb von i64 weist die Bridge ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
