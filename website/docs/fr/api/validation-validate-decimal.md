---
locale: fr
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
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

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
