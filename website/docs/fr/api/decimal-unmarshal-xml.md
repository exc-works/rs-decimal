---
locale: fr
source_revision: 1640bba425b4f674d29468bfc500f4a4319c227e5d4c34f98666dafad0d9b981
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_xml"
---
# Decimal::unmarshal_xml

Décoder xml dans une valeur existante.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

source : &str/string ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige xml. Décode le premier élément XML en combinant son texte direct, CDATA et les références d’entités décodées. Le texte des enfants imbriqués est ignoré ; le texte avant et après un enfant est concaténé. Les éléments suivants sont ignorés. Le cœur valide les noms, attributs, références de caractères et balises de fermeture correspondantes. Un contenu vide après suppression des espaces réinitialise le récepteur à un zéro non initialisé ; cela diffère de YAML null, qui le laisse inchangé. Un décodage réussi d’une entrée autre que null initialise le décimal et conserve l’échelle des valeurs numériquement non nulles. Les erreurs préservent tous les champs du récepteur et les catégories d’erreur de bibliothèque, dont Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
