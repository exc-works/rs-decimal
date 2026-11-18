---
locale: fr
source_revision: d38eb31907db7ef2944305de43af506a13542551be2fcba604fea1ed0a54e78f
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

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
