---
locale: fr
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

Obtenir le contenu sémantique yaml.

```rust
fn marshal_yaml (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Rust exige la fonctionnalité yaml, activée dans le navigateur. Renvoie une chaîne scalaire YAML sémantique, pas un document YAML sérialisé. Transmettez-la comme chaîne à un générateur YAML pour que les guillemets et étiquettes soient correctement traités. Le texte décimal canonique supprime les zéros fractionnaires finaux. Le zéro par défaut non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
