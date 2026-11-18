---
locale: fr
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

Inspecter l’enregistrement d’échec de validation.

```rust
pub struct ValidationError
```

## Paramètres, résultat et utilisation courante

Enregistrement d’échec avec les propriétés publiques field, tag et parameter. ValidationRule.validate le renvoie en cas d’échec ; validate_decimal renvoie une liste. JavaScript expose des enregistrements modifiables adaptés à la traduction. Ce ne sont ni des catégories DecimalError ni des messages d’exception. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
