---
locale: fr
source_revision: e175d4d7de2a038dff837a099025c66d8337b4480eacf9bc8d2de1f1d5950025
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

Évaluer une règle décimale exacte.

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

tag : &str/string, parameter : &str/string ; renvoie `Result<ValidationRule, DecimalError>`/un handle de règle. Utilisez les étiquettes complètes comme decimal_gt. Les paramètres de comparaison sont du texte décimal ; between prend lower~upper ; max_precision prend un isize non négatif. Les étiquettes inconnues, paramètres mal formés et intervalles inversés renvoient InvalidArgument. En wasm32, isize est signé sur 32 bits. Les règles sans paramètre conservent le texte du paramètre sans l’interpréter. Les règles comparent exactement les valeurs Decimal sans conversion flottante. decimal_required vérifie l’initialisation : un zéro initialisé passe donc le test. eq/ne/gt/gte/lt/lte comparent numériquement, même pour un zéro non initialisé. between utilise lower~upper avec bornes incluses et exige en plus l’initialisation. positive/negative/nonzero exigent l’initialisation. max_precision vérifie l’échelle stockée, y compris les zéros finaux conservés ; cela n’implique pas required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
