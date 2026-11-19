---
locale: fr
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

Encoder NullDecimal en octets json.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Vec<u8>`/Uint8Array contenant des octets JSON. Rust exige la fonctionnalité json, activée dans le build du navigateur. Les valeurs nullables invalides sont encodées en JSON null ; les valeurs valides en décimaux canoniques entre guillemets, même si le Decimal interne n’est pas initialisé. Les octets renvoyés sont une copie indépendante et la lecture ne modifie pas l’état.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="fr" />

## Méthodes associées

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
