---
locale: fr
source_revision: 8e284228528617403ea37afb52c965d21d076aee976dac5cd366e305507e8e05
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_xml_attr"
---
# Decimal::unmarshal_xml_attr

Décoder xml_attr dans une valeur existante.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &str/string ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige xml. Fournissez la valeur d’attribut dont les entités sont déjà décodées, pas un fragment d’attribut XML : "1.23" est valide, mais `"&#49;.23"` n’est pas décodé à nouveau. Supprime les espaces aux extrémités et analyse le texte décimal exact. Un contenu vide réinitialise le récepteur à un zéro non initialisé. Un décodage réussi d’une entrée autre que null initialise le décimal et conserve l’échelle des valeurs numériquement non nulles. Les erreurs préservent tous les champs du récepteur et les catégories d’erreur de bibliothèque, dont Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml-attr" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
