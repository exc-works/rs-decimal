---
locale: fr
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

Inspecter l’enregistrement d’échec de validation.

```rust
parameter: String
```

## Paramètres, résultat et utilisation courante

String/string publique possédée conservant exactement le paramètre de règle d’origine, espaces compris. Il est substitué une seule fois à {1}, sans interprétation récursive du modèle. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
