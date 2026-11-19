---
locale: fr
source_revision: 87ff1c64a9a30b8c362e1bf8438b7f751aeb4f561a2bd311095a9f2b86ed45db
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

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
