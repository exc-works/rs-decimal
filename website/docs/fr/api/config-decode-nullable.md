---
locale: fr
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
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

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
