---
locale: fr
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
