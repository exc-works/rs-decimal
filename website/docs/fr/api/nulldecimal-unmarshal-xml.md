---
locale: fr
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

Décoder xml dans une valeur existante.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

source : &str/string ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige xml. Décode le premier élément XML en combinant son texte direct, CDATA et les références d’entités décodées. Le texte des enfants imbriqués est ignoré ; le texte avant et après un enfant est concaténé. Les éléments suivants sont ignorés. Le cœur valide les noms, attributs, références de caractères et balises de fermeture correspondantes. Un contenu vide après suppression des espaces efface valid et réinitialise le Decimal interne à un zéro non initialisé. Un décodage réussi d’une entrée autre que null initialise le décimal et conserve l’échelle des valeurs numériquement non nulles. Définit valid=true. Les erreurs préservent tous les champs du récepteur et les catégories d’erreur de bibliothèque, dont Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
