---
locale: fr
source_revision: 6fae0dc88a563ce1dfaf67cbb1ece08366df9aad8969e9320dabf2cf06fa5933
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

Inspecter l’enregistrement d’échec de validation.

```rust
tag: & 'static str
```

## Paramètres, résultat et utilisation courante

Chaîne statique publique Rust identifiant la règle en échec. Le navigateur utilise une propriété de chaîne possédée ; les étiquettes inconnues utilisent le modèle par défaut du cœur "{0} is invalid" lors de la traduction. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
