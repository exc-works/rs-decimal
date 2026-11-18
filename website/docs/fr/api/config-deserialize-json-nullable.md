---
locale: fr
source_revision: a76fd60d6946476bbdf689a1364eae83f451fe65b731c9c72ed5e79487241605
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

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
