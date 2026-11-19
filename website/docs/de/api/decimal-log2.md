---
locale: de
source_revision: a329356af92d18ac765420533258b5c608220d0cf27b5772304a0e1b5b8d5f9e
translation_provenance: en-US-german-reviewed
api_id: Decimal::log2
---
# Decimal::log2

Den Zweierlogarithmus berechnen. Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine strukturierte Ausnahme. Keine Parameter. Die Standardausgabepräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Die Eingabe muss positiv sein; null und negative Eingaben liefern InvalidLog. Exakte Potenzen der Basis haben exakte ganzzahlige Logarithmen, die bei der Ausgabeskala erhalten bleiben. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn log2 (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2" locale="de" />

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
