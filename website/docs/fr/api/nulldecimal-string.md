---
locale: fr
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
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

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
