---
locale: fr
source_revision: b2646b52b58adc30058be6fedbac290221f3b2b947946b06b25fe8783f697afc
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

Décoder une entrée json dans un Decimal existant.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

bytes : &[u8]/Uint8Array ; modifie le récepteur et renvoie Result<(), DecimalError>/undefined. Rust exige la fonctionnalité json. Fournissez les octets JSON UTF-8 d’origine : les nombres sont validés sans aller-retour flottant, et les chaînes sont déséchappées avant l’analyse décimale exacte. Ne convertissez pas d’abord un jeton numérique en number JavaScript avec JSON.parse. Les espaces JSON sont uniquement espace/tab/CR/LF. JSON null laisse l’ensemble du Decimal existant inchangé, y compris son initialisation. Une entrée autre que null, analysée avec succès, initialise le décimal, conserve l’échelle des valeurs numériquement non nulles et normalise le zéro analysé à l’échelle zéro. Les erreurs préservent l’état complet du récepteur. Les erreurs de décodage de la bibliothèque conservent Unmarshal et les catégories sous-jacentes ; les erreurs de ressources surviennent aussi avant la modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
