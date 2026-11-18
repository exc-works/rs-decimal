---
locale: fr
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

Sélectionner une entrée de configuration Bytes.

```rust
Bytes (& 'a [u8])
```

## Paramètres, résultat et utilisation courante

Emprunte des octets contenant du texte décimal, et non une charge utile décimale binaire. Les modifications du Uint8Array dans le navigateur avant le décodage sont observées ; le décodage ne modifie jamais le tableau. Un UTF-8 incorrect fait échouer l’analyse décimale.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
