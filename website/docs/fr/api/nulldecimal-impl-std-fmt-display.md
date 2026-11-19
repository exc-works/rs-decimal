---
locale: fr
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

Formater une valeur nullable avec Rust Display.

```rust
impl std :: fmt :: Display for NullDecimal
```

## Paramètres, résultat et utilisation courante

Display délègue à string() du nullable : texte canonique interne si la valeur est valide, null sinon. Le formateur écrit du texte, pas un jeton JSON ni un tampon d’octets. format! appelle Display::fmt ; cette syntaxe de trait est exécutée nativement et n’est pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
