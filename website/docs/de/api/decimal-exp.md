---
locale: de
source_revision: 4812fffbe74c100e5632736fe3ca1c5dd726efa9358bb2ca70cc5db51e55bd14
translation_provenance: en-US-german-reviewed
api_id: Decimal::exp
---
# Decimal::exp

Die Exponentialfunktion e^x berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. Keine Parameter. Die Standardausgabepräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Der Kern erlaubt alle reellen Eingaben. exp(0) ist initialisierte eins; ein ausreichend negatives Argument kann bei angeforderter Präzision korrekt auf initialisierte Null runden. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="de" />

## Grenzen im Browser

Diese Gruppe hat zusätzliche [Ressourcengrenzen](../resource-policy): Ausgabepräzision höchstens 128, Eingabeskala höchstens 256 und Eingabekoeffizient höchstens 4096 Bits. Gewöhnliche exp-Auswertung verlangt außerdem einen Eingabebetrag von höchstens 64; negative Eingaben, deren direkte Rundung auf null der Kern beweist, behalten den schnellen Pfad. Große Eingabeskala kann bei parameterlosen Methoden eine Standardpräzision über 128 wählen; wählen Sie mit with_prec explizit eine erlaubte Ausgabepräzision. Ressourcenfehler unterscheiden sich von NoConvergence im Kern. Stop unterbricht den Lauf; der nächste Lauf erzeugt eine frische Wasm-Instanz.

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
