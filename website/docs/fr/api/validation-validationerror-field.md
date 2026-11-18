---
locale: fr
source_revision: 9d0e7a5134f00bf353febb8beb1a65cb4dacb67d9c7e5b41dbb343b7baa674b0
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

Inspecter l’enregistrement d’échec de validation.

```rust
field: String
```

## Paramètres, résultat et utilisation courante

String/string publique possédée contenant le nom de champ fourni par l’appelant. Ce n’est ni un chemin d’accès à une propriété ni un fragment HTML. Les accolades sont insérées littéralement lors de la traduction. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
