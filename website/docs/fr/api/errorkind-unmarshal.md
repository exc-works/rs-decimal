---
locale: fr
source_revision: c870cbfccba48189ea2c1f73c60ffc0835f379cdb13fc7e1adf9905fa90662b8
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Identifier les erreurs Unmarshal.

```rust
Unmarshal
```

## Paramètres, résultat et utilisation courante

Le décodage a échoué. Une autre catégorie, comme InvalidFormat, peut en préciser la cause ; testez l’appartenance plutôt que de supposer une catégorie unique. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
