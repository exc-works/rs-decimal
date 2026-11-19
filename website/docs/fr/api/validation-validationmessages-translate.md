---
locale: fr
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

Traduire les échecs de validation avec des modèles possédés.

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## Paramètres, résultat et utilisation courante

errors : &[ValidationError]/tableau ; renvoie un tableau de chaînes dans le même ordre. La correspondance de langue ignore la casse, accepte les tirets, les traits de soulignement et les suffixes langue-région ; zh-Hant/zh-TW choisissent le chinois traditionnel. en-US/en-GB utilisent l’anglais ; une langue inconnue revient à l’anglais. {0} insère le champ et {1} le paramètre d’origine en un seul passage ; les accolades insérées ne sont jamais développées à nouveau. Les chaînes renvoyées sont du texte brut ; affichez-les comme du texte, pas du HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
