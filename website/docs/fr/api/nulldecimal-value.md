---
locale: fr
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

Obtenir la valeur SQL canonique.

```rust
fn value (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Les valeurs nullables invalides renvoient None/null, quelles que soient les données internes masquées. Les valeurs valides renvoient le texte décimal canonique, y compris "0" pour un zéro interne non initialisé. Supprime les zéros fractionnaires finaux sans changer l’échelle ni l’initialisation. Le texte renvoyé est une valeur scalaire, pas un fragment de requête SQL entre guillemets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
