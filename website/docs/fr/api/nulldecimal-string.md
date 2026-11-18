---
locale: fr
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

Lire le texte canonique nullable.

```rust
fn string (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Les valeurs valides utilisent le texte canonique du Decimal interne, sans zéros fractionnaires finaux. Les valeurs invalides renvoient le littéral null, quelles que soient les données internes masquées. Cela diffère de marshal_text, qui renvoie des octets vides pour les valeurs invalides. La lecture ne change ni la validité ni l’initialisation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="fr" />

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
