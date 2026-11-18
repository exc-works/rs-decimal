---
locale: fr
source_revision: 4b5fc87cb7547a8cb739a71f9c75b83ad78ac1a07faee17459dcd903e68f40c4
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Utiliser l’implémentation Rust Eq.

```rust
impl Eq for validation::ValidationError
```

## Paramètres, résultat et utilisation courante

L’égalité de ValidationError compare ensemble field, tag et le paramètre d’origine ; elle ne compare pas seulement les messages traduits. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
