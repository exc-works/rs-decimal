---
locale: de
source_revision: 06880d717589581395da1d4538089b3e6373aa9e862e6934eb18c5e117dffc70
translation_provenance: en-US-german-reviewed
api_id: Decimal::ln
---
# Decimal::ln

Den natürlichen Logarithmus berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. Keine Parameter. Die Standardausgabepräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. ln(1) ist exakt null. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn ln (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln" locale="de" />

## Grenzen im Browser

Diese Gruppe hat zusätzliche [Ressourcengrenzen](../resource-policy): Ausgabepräzision höchstens 128, Eingabeskala höchstens 256 und Eingabekoeffizient höchstens 4096 Bits. Gewöhnliche exp-Auswertung verlangt außerdem einen Eingabebetrag von höchstens 64; negative Eingaben, deren direkte Rundung auf null der Kern beweist, behalten den schnellen Pfad. Große Eingabeskala kann bei parameterlosen Methoden eine Standardpräzision über 128 wählen; wählen Sie mit with_prec explizit eine erlaubte Ausgabepräzision. Ressourcenfehler unterscheiden sich von NoConvergence im Kern. Stop unterbricht den Lauf; der nächste Lauf erzeugt eine frische Wasm-Instanz.

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
