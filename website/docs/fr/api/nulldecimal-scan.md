---
locale: fr
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
