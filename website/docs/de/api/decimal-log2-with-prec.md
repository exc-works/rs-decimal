---
locale: de
source_revision: f3a534483ccc1e91ef144a2c4109aaf3dd71447ded56bb396f1a854b09445c8e
translation_provenance: en-US-german-reviewed
api_id: Decimal::log2_with_prec
---
# Decimal::log2_with_prec

Den Zweierlogarithmus berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. prec: nichtnegatives isize, als ganzzahlige number dargestellt, wählt Nachkommastellen; negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. Exakte Potenzen der Basis haben exakte ganzzahlige Logarithmen, die bei der Ausgabeskala erhalten bleiben. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn log2_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2-with-prec" locale="de" />

## Grenzen im Browser

Diese Gruppe hat zusätzliche [Ressourcengrenzen](../resource-policy): Ausgabepräzision höchstens 128, Eingabeskala höchstens 256 und Eingabekoeffizient höchstens 4096 Bits. Gewöhnliche exp-Auswertung verlangt außerdem einen Eingabebetrag von höchstens 64; negative Eingaben, deren direkte Rundung auf null der Kern beweist, behalten den schnellen Pfad. Große Eingabeskala kann bei parameterlosen Methoden eine Standardpräzision über 128 wählen; wählen Sie mit with_prec explizit eine erlaubte Ausgabepräzision. Ressourcenfehler unterscheiden sich von NoConvergence im Kern. Stop unterbricht den Lauf; der nächste Lauf erzeugt eine frische Wasm-Instanz.

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
