---
locale: fr
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
translation_provenance: en-US-french-reviewed
api_id: "validation"
---
# validation

Recueillir les échecs de règles dans l’ordre et les traduire.

```rust
pub mod validation
```

## Paramètres, résultat et utilisation courante

Rust exige la fonctionnalité validation, activée dans le build du navigateur. Ce module regroupe l’évaluation exacte des règles et les modèles de messages possédés, sans framework ni registre global. Dans le navigateur, utilisez l’espace de noms validation. Les noms de champs sont du texte fourni par l’appelant ; la traduction les insère littéralement. Utilisez explicitement required si une entrée absente doit échouer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
