---
locale: fr
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

Choisir la variante d’entrée SQL Float32.

```rust
Float32 (f32)
```

## Paramètres, résultat et utilisation courante

Transporte binary32. Le navigateur accepte number et Rust le réduit en f32 avant scan. NaN, les infinis et un dépassement lors de la réduction produisent Unmarshal. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
