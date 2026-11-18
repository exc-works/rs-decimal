---
locale: fr
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

Évaluer une règle décimale exacte.

```rust
fn matches (& self , value : & Decimal) -> bool
```

## Paramètres, résultat et utilisation courante

value : &Decimal/Decimal ; renvoie bool/boolean et préserve la règle comme la valeur. Les règles comparent exactement les valeurs Decimal sans conversion flottante. decimal_required vérifie l’initialisation : un zéro initialisé passe donc le test. eq/ne/gt/gte/lt/lte comparent numériquement, même pour un zéro non initialisé. between utilise lower~upper avec bornes incluses et exige en plus l’initialisation. positive/negative/nonzero exigent l’initialisation. max_precision vérifie l’échelle stockée, y compris les zéros finaux conservés ; cela n’implique pas required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
