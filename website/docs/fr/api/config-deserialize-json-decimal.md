---
locale: fr
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
translation_provenance: en-US-french-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

Désérialiser un champ de configuration JSON exact.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## Paramètres, résultat et utilisation courante

deserializer : D implémentant Serde Deserializer ; destiné à #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. Exige une source serde_json et la fonctionnalité config. Conserve les jetons numériques originaux sans aller-retour flottant. Le navigateur fournit un adaptateur concret recevant une chaîne source contenant un scalaire JSON complet, pas un désérialiseur arbitraire. JSON null et les chaînes vides sont rejetés pour la cible non nullable. Les erreurs ont le type d’erreur du désérialiseur Serde ; dans le navigateur, category="serde" conserve leur message au lieu d’inventer des catégories DecimalError. Toute entrée finale autre que des espaces JSON est rejetée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
