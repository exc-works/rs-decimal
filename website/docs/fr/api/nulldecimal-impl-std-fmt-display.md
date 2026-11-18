---
locale: fr
source_revision: 425ca68f655481c858e4668ea2e1df0ddc734454c90f72d356190ed7ac4eb22e
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

## Limites du navigateur

L’entrée du décodeur est limitée à 8192 octets ; la longueur des chaînes de paramètres est aussi contrôlée avant copie dans Wasm. Rust vérifie à nouveau la [politique de ressources décimales](../resource-policy) sur le texte et les chaînes JSON décodées : les échappements ne peuvent donc pas contourner les limites d’exposant ou d’échelle. Les diagnostics UTF-8, JSON et de syntaxe décimale incorrects appartiennent au cœur. Les erreurs d’entrée ou de ressources ne peuvent pas modifier partiellement le récepteur ; les objets enveloppes déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
