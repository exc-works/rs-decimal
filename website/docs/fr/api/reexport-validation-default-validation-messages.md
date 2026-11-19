---
locale: fr
source_revision: c829b94ceed61e0a65772da94750a53ecb8269151bdb6f1efd6a7c407ad09f12
translation_provenance: en-US-french-reviewed
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

Obtenir une nouvelle table de modèles pour une langue.

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## Paramètres, résultat et utilisation courante

locale : &str/string ; renvoie `BTreeMap<String,String>`/un objet ordinaire contenant les douze modèles de règles. Les langues inconnues utilisent l’anglais. Chaque appel possède sa table ; les modifications n’affectent donc ni les appels suivants ni les autres objets ValidationMessages. La réexportation est le chemin d’import du module public pour la même fonction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
