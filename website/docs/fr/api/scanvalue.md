---
locale: fr
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
translation_provenance: en-US-french-reviewed
api_id: "ScanValue"
---
# ScanValue

Choisir une entrée SQL typée indépendante du pilote.

```rust
pub enum ScanValue< 'a >
```

## Paramètres, résultat et utilisation courante

L’énumération distingue NULL, f32, f64, i64, le texte, le texte en octets et les valeurs non prises en charge. Choisissez explicitement une variante avant d’appeler Decimal.scan ou NullDecimal.scan. En Rust, le texte ou les octets empruntés doivent seulement rester valides pendant l’appel ; dans le navigateur, Bytes conserve le tableau de l’appelant et observe les modifications effectuées avant scan. Aucun service de base de données n’est contacté.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
