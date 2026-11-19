---
locale: de
source_revision: 07fd0624795c1d5a39d29225db8108c9026886d41981fb4ff89e2fb3be01274d
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

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
