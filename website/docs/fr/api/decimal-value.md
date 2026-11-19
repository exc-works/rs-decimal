---
locale: fr
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
translation_provenance: en-US-french-reviewed
api_id: "Decimal::value"
---
# Decimal::value

Obtenir la valeur SQL canonique.

```rust
fn value (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Même un Decimal non initialisé renvoie "0", qui n’est pas NULL ; utilisez NullDecimal si SQL NULL est nécessaire. Supprime les zéros fractionnaires finaux sans changer l’échelle ni l’initialisation. Le texte renvoyé est une valeur scalaire, pas un fragment de requête SQL entre guillemets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
