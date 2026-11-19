---
locale: fr
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: en-US-french-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

Décoder une valeur de configuration facultative.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : ConfigValue ; renvoie `Result<NullDecimal, DecimalError>`/un nouveau handle nullable. Null, Text exactement vide et Bytes vide deviennent invalides avec un zéro interne non initialisé. Un Text composé uniquement d’espaces et un Number vide échouent encore à l’analyse décimale. Les Nullable existants clonent à la fois la validité et les données internes masquées, même lorsqu’ils sont invalides ; un Decimal emprunté devient valide sans forcer son initialisation. Les autres entrées suivent decode_decimal et deviennent valides en cas de succès. Les entrées ne sont pas consommées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
