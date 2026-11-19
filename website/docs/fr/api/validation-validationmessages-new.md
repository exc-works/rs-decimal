---
locale: fr
source_revision: 2b24eac398b765662bae8967979a2547332ee566c8d7e9c0bb785ba0d1f9e478
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

Traduire les échecs de validation avec des modèles possédés.

```rust
fn new (locale : & str) -> Self
```

## Paramètres, résultat et utilisation courante

locale : &str/string ; renvoie de nouveaux modèles de messages possédés. La correspondance de langue ignore la casse, accepte les tirets, les traits de soulignement et les suffixes langue-région ; zh-Hant/zh-TW choisissent le chinois traditionnel. en-US/en-GB utilisent l’anglais ; une langue inconnue revient à l’anglais. {0} insère le champ et {1} le paramètre d’origine en un seul passage ; les accolades insérées ne sont jamais développées à nouveau. Les chaînes renvoyées sont du texte brut ; affichez-les comme du texte, pas du HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
