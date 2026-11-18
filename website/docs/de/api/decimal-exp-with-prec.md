---
locale: de
source_revision: 4817d408a3e6bcf6471a527b25ba77c1f63c6984d0953a16de68bfe470c82e92
translation_provenance: en-US-german-reviewed
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

Die Exponentialfunktion e^x berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. prec: nichtnegatives isize, als ganzzahlige number dargestellt, wählt Nachkommastellen; negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Der Kern erlaubt alle reellen Eingaben. exp(0) ist initialisierte eins; ein ausreichend negatives Argument kann bei angeforderter Präzision korrekt auf initialisierte Null runden. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="de" />

## Grenzen im Browser

Diese Gruppe hat zusätzliche [Ressourcengrenzen](../resource-policy): Ausgabepräzision höchstens 128, Eingabeskala höchstens 256 und Eingabekoeffizient höchstens 4096 Bits. Gewöhnliche exp-Auswertung verlangt außerdem einen Eingabebetrag von höchstens 64; negative Eingaben, deren direkte Rundung auf null der Kern beweist, behalten den schnellen Pfad. Große Eingabeskala kann bei parameterlosen Methoden eine Standardpräzision über 128 wählen; wählen Sie mit with_prec explizit eine erlaubte Ausgabepräzision. Ressourcenfehler unterscheiden sich von NoConvergence im Kern. Stop unterbricht den Lauf; der nächste Lauf erzeugt eine frische Wasm-Instanz.

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
