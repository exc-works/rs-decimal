---
locale: fr
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
translation_provenance: en-US-french-reviewed
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

Consulter la révision Go amont utilisée pour les contrôles de compatibilité.

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## Utilisation courante et sémantique

Cette &str immuable est une révision du code source, pas le commit du build Rust ou web actuel. Le navigateur lit la même constante depuis le Rust compilé. Ne l’interprétez pas comme une version de paquet ou un identifiant de déploiement.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
