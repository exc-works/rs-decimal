---
locale: fr
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
translation_provenance: en-US-french-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

Obtenir une nouvelle table de modèles pour une langue.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## Paramètres, résultat et utilisation courante

locale : &str/string ; renvoie `BTreeMap<String,String>`/un objet ordinaire contenant les douze modèles de règles. Les langues inconnues utilisent l’anglais. Chaque appel possède sa table ; les modifications n’affectent donc ni les appels suivants ni les autres objets ValidationMessages. La réexportation est le chemin d’import du module public pour la même fonction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
