---
locale: fr
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
translation_provenance: en-US-french-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

Recueillir les échecs de règles dans l’ordre et les traduire.

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## Paramètres, résultat et utilisation courante

Rust exige la fonctionnalité validation, activée dans le build du navigateur. field : &str/string, value : &Decimal/Decimal, rules : &[ValidationRule]/tableau ; renvoie chaque ValidationError en échec dans l’ordre de l’appelant. Des règles répétées peuvent produire des échecs répétés ; aucune réflexion, aucun registre ni arrêt anticipé n’est utilisé. Des règles vides renvoient un tableau vide. Toutes les entrées sont empruntées. Les noms de champs sont du texte fourni par l’appelant ; la traduction les insère littéralement. Utilisez explicitement required si une entrée absente doit échouer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
