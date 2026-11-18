---
locale: fr
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

Valider un champ selon une règle.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## Paramètres, résultat et utilisation courante

field : &str/string, value : &Decimal/Decimal ; renvoie `Result<(), ValidationError>`. Une réussite vaut undefined en JavaScript. Un échec lève une ValidationError JavaScript avec category="validation" et les propriétés field/tag/parameter ; elle est distincte de DecimalError. Le texte du champ et du paramètre d’origine reste inchangé, accolades et espaces compris. La règle et la valeur sont empruntées. Utilisez validate_decimal pour recueillir plusieurs échecs sans lever d’exception.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
