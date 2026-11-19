---
locale: fr
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

Remplacer les modèles anglais de validation.

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

messages : `BTreeMap<String,String>`/un objet à valeurs de chaînes ; renvoie `Result<ValidationMessages, DecimalError>`/un handle de messages. Remplace uniquement les clés fournies ; toutes les autres règles conservent les valeurs anglaises par défaut. Les clés autorisées sont les douze règles decimal_*. Les modèles vides, clés inconnues, accolades non appariées et espaces réservés autres que {0}/{1} renvoient InvalidArgument. La répétition d’espaces réservés valides est autorisée. Ne modifie aucun modèle global ni précédemment construit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
