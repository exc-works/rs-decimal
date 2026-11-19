---
locale: fr
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

Obtenir le contenu sémantique yaml.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Rust exige la fonctionnalité yaml, activée dans le navigateur. Renvoie une chaîne scalaire YAML sémantique, pas un document YAML sérialisé. Transmettez-la comme chaîne à un générateur YAML pour que les guillemets et étiquettes soient correctement traités. Le texte décimal canonique supprime les zéros fractionnaires finaux. Une valeur nullable invalide renvoie None/null, qui représente un scalaire YAML null ; une valeur valide avec un zéro interne non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité ; le navigateur représente None de Rust par null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
