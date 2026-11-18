---
locale: fr
source_revision: 559007df6751f5417671f4f4b9a33c4ae6759ecf82dbd4fbeb7d9f0c40061e1a
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

Lire un scalaire SQL indépendant du pilote.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : ScanValue ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Choisissez explicitement la variante : Int64 utilise un bigint exact ; Float32 réduit en binary32 ; Text/Bytes conservent les chiffres décimaux d’origine. Le texte SQL peut être entouré d’une paire correspondante de guillemets doubles, retirés sans déséchappement JSON. NULL efface la validité et réinitialise le Decimal interne à un zéro non initialisé ; une entrée autre que NULL traitée avec succès définit valid=true. Les types non pris en charge et les flottants non finis renvoient Unmarshal. Les autres échecs de décodage conservent les catégories sous-jacentes. Toutes les erreurs préservent l’état du récepteur. Cet adaptateur ne nécessite aucune connexion à une base de données.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
