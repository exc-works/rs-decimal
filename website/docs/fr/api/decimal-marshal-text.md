---
locale: fr
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Encoder Decimal en octets text.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Vec<u8>`/Uint8Array contenant le texte décimal ASCII canonique. Les zéros fractionnaires finaux sont supprimés. Le zéro Decimal par défaut produit le caractère ASCII 0. Les octets renvoyés sont une copie indépendante et la lecture ne modifie pas l’état.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
