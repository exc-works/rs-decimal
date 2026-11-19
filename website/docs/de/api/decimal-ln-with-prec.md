---
locale: de
source_revision: 14996308295f3d9761be44a618304820607fea78ed2b85488ce4a81975821f38
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

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
