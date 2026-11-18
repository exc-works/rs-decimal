---
locale: fr
source_revision: 0ee4a72ff112d52d1fe74af33099195201a24147830d9b414e5d9feccaa1d9af
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::impl Debug"
---
# validation::ValidationMessages::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for validation::ValidationMessages
```

## Paramètres, résultat et utilisation courante

Debug produit un texte de diagnostic, pas un format sérialisé stable de validation. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-debug" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
