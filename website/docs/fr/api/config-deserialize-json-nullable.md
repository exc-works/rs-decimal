---
locale: fr
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
translation_provenance: en-US-french-reviewed
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

Désérialiser un champ de configuration JSON exact.

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## Paramètres, résultat et utilisation courante

deserializer : D implémentant Serde Deserializer ; destiné à #[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")]. Exige une source serde_json et la fonctionnalité config. Conserve les jetons numériques originaux sans aller-retour flottant. Le navigateur fournit un adaptateur concret recevant une chaîne source contenant un scalaire JSON complet, pas un désérialiseur arbitraire. JSON null et une chaîne JSON exactement vide deviennent invalides ; les chaînes composées uniquement d’espaces échouent encore. Les valeurs non vides décodées avec succès deviennent valides. Les erreurs ont le type d’erreur du désérialiseur Serde ; dans le navigateur, category="serde" conserve leur message au lieu d’inventer des catégories DecimalError. Toute entrée finale autre que des espaces JSON est rejetée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
