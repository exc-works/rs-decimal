---
locale: fr
source_revision: 8e304b6c42ea7cec512b7054ed8c7c7654d4540ebdd19dbc647e83dc885c5809
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

Choisir la variante d’entrée SQL Null.

```rust
Null
```

## Paramètres, résultat et utilisation courante

Représente SQL NULL ; scan réinitialise Decimal à un zéro non initialisé et efface NullDecimal. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
