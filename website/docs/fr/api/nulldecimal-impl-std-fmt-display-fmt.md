---
locale: fr
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

Formater une valeur nullable avec Rust Display.

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## Paramètres, résultat et utilisation courante

Display délègue à string() du nullable : texte canonique interne si la valeur est valide, null sinon. Le formateur écrit du texte, pas un jeton JSON ni un tampon d’octets. format! appelle Display::fmt ; cette syntaxe de trait est exécutée nativement et n’est pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
