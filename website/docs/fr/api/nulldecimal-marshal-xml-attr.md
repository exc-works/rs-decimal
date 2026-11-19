---
locale: fr
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

Obtenir le contenu sémantique xml_attr.

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Rust exige la fonctionnalité xml, activée dans le navigateur. Renvoie le contenu d’attribut déjà décodé ; l’appelant fournit le nom d’attribut et l’échappement XML. Le texte décimal canonique supprime les zéros fractionnaires finaux. Une valeur nullable invalide renvoie None/null : omettez l’attribut au lieu d’écrire un attribut vide. Une valeur valide avec un zéro interne non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité ; le navigateur représente None de Rust par null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
