---
locale: de
source_revision: d7c263dcbf87f527f8688d8c67f916ab9df964fed5eeb802c12da2b176a5e991
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

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
