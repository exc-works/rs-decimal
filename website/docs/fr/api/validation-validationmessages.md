---
locale: fr
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

Traduire les échecs de validation avec des modèles possédés.

```rust
pub struct ValidationMessages
```

## Paramètres, résultat et utilisation courante

Possède les modèles servant à traduire les enregistrements ValidationError. Utilisez new(locale) ou with_messages(map) ; modifier une autre table de modèles n’affecte pas ce handle. La correspondance de langue ignore la casse, accepte les tirets, les traits de soulignement et les suffixes langue-région ; zh-Hant/zh-TW choisissent le chinois traditionnel. en-US/en-GB utilisent l’anglais ; une langue inconnue revient à l’anglais. {0} insère le champ et {1} le paramètre d’origine en un seul passage ; les accolades insérées ne sont jamais développées à nouveau. Les chaînes renvoyées sont du texte brut ; affichez-les comme du texte, pas du HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
