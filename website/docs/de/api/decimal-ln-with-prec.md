---
locale: de
source_revision: cd679689311c552a10e0f46bebe2c6d4b3b7c9ae648dc393862458b144cd8b20
translation_provenance: en-US-german-reviewed
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

Den natürlichen Logarithmus berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. prec: nichtnegatives isize, als ganzzahlige number dargestellt, wählt Nachkommastellen; negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. ln(1) ist exakt null. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="de" />

## Grenzen im Browser

Diese Gruppe hat zusätzliche [Ressourcengrenzen](../resource-policy): Ausgabepräzision höchstens 128, Eingabeskala höchstens 256 und Eingabekoeffizient höchstens 4096 Bits. Gewöhnliche exp-Auswertung verlangt außerdem einen Eingabebetrag von höchstens 64; negative Eingaben, deren direkte Rundung auf null der Kern beweist, behalten den schnellen Pfad. Große Eingabeskala kann bei parameterlosen Methoden eine Standardpräzision über 128 wählen; wählen Sie mit with_prec explizit eine erlaubte Ausgabepräzision. Ressourcenfehler unterscheiden sich von NoConvergence im Kern. Stop unterbricht den Lauf; der nächste Lauf erzeugt eine frische Wasm-Instanz.

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
