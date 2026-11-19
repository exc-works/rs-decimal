---
locale: fr
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
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

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
