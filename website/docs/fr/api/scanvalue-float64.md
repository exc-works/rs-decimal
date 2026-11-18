---
locale: fr
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

Choisir la variante d’entrée SQL Float64.

```rust
Float64 (f64)
```

## Paramètres, résultat et utilisation courante

Transporte binary64. Les valeurs finies utilisent la conversion décimale la plus courte du cœur ; NaN et les infinis renvoient Unmarshal. Des number JavaScript déjà arrondis ne permettent pas de retrouver les chiffres d’origine. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
