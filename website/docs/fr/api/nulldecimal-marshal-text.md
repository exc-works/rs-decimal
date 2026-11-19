---
locale: fr
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

Encoder NullDecimal en octets text.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Vec<u8>`/Uint8Array contenant le texte décimal ASCII canonique. Les valeurs valides encodent le texte décimal canonique ; les valeurs invalides des octets vides, pas le mot null. Les octets renvoyés sont une copie indépendante et la lecture ne modifie pas l’état.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
