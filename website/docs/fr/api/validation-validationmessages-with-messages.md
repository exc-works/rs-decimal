---
locale: fr
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

Remplacer les modèles anglais de validation.

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

messages : `BTreeMap<String,String>`/un objet à valeurs de chaînes ; renvoie `Result<ValidationMessages, DecimalError>`/un handle de messages. Remplace uniquement les clés fournies ; toutes les autres règles conservent les valeurs anglaises par défaut. Les clés autorisées sont les douze règles decimal_*. Les modèles vides, clés inconnues, accolades non appariées et espaces réservés autres que {0}/{1} renvoient InvalidArgument. La répétition d’espaces réservés valides est autorisée. Ne modifie aucun modèle global ni précédemment construit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
