---
locale: de
source_revision: de92d5e22d11b6fc714d2080362ae5ea68f8d14531ba18de4922662c7652ab91
translation_provenance: en-US-german-reviewed
api_id: Decimal::log2_with_prec
---
# Decimal::log2_with_prec

Den Zweierlogarithmus berechnen. prec: nichtnegatives isize, als ganzzahlige number dargestellt, wählt Nachkommastellen; negative Präzision liefert InvalidPrecision vor Definitionsbereichsprüfungen. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. Exakte Potenzen der Basis haben exakte ganzzahlige Logarithmen, die bei der Ausgabeskala erhalten bleiben. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn log2_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2-with-prec" locale="de" />

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
