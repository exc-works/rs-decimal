---
locale: de
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: en-US-german-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Den Ganzzahlteil durch Abschneiden in Richtung null extrahieren.

```rust
fn int_part (& self) -> BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert BigInt, die Bridge bigint, ohne Gleitkommakonvertierung. -12.75 ergibt -12, nicht floor(-12.75) = -13. Die Standardnull ergibt null. Das Beispiel serialisiert die Ganzzahl als Zeichenfolge, um beliebige Genauigkeit in JSON zu erhalten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="de" />

## Grenzen im Browser

Eingaben werden ausgeliehen und bleiben nach Erfolg oder gewöhnlichen Fehlern nutzbar. Zurückgegebene Decimal-Werte besitzen unabhängige Daten; Rechenergebnisse sind initialisiert. Vor dem Kernaufruf prüft der Browser die [Ressourcenrichtlinie](../resource-policy): Skalenausrichtung, Zwischenprodukte, Divisionsskalierung und Ausgabegenauigkeit. Konservative Vorprüfungen können Berechnungen mit kleinen Endergebnissen ablehnen. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt; ein Panic beendet den Lauf.

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
