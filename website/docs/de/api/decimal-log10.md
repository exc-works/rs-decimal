---
locale: de
source_revision: ea6f97f51071f45a8c9bf2f4df22faacd07f9c9b025ed9b3ebf43ae9fab4e095
translation_provenance: en-US-german-reviewed
api_id: Decimal::log10
---
# Decimal::log10

Den Zehnerlogarithmus berechnen. Keine Parameter. Die Standardausgabepräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. Exakte Potenzen der Basis haben exakte ganzzahlige Logarithmen, die bei der Ausgabeskala erhalten bleiben. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn log10 (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10" locale="de" />

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
