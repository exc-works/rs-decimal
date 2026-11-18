---
locale: fr
source_revision: 9f6709052cffc0b28f6c72e94064cf76722563f48008476b3a2d47db292f5bda
translation_provenance: en-US-french-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

Obtenir une nouvelle table de modèles pour une langue.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## Paramètres, résultat et utilisation courante

locale : &str/string ; renvoie `BTreeMap<String,String>`/un objet ordinaire contenant les douze modèles de règles. Les langues inconnues utilisent l’anglais. Chaque appel possède sa table ; les modifications n’affectent donc ni les appels suivants ni les autres objets ValidationMessages. La réexportation est le chemin d’import du module public pour la même fonction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="fr" />

## Limites du navigateur

La fonctionnalité validation est activée. Chaque chaîne est limitée à 8192 octets UTF-8 ; les listes de règles/erreurs et les tables de messages à 64 entrées ; le transport JSON à 65536 octets. Les paramètres décimaux des règles suivent la [politique de ressources](../resource-policy). La traduction contrôle préalablement une estimation conservatrice de 65536 octets pour le texte total après substitution et peut rejeter un résultat qui tiendrait exactement. Les étiquettes et champs personnalisés sont des données, jamais du code exécuté. Les échecs de validation, erreurs de configuration et erreurs de ressources du bridge restent distincts.

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
