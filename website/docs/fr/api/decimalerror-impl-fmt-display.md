---
locale: fr
source_revision: 313aa913e617945bd29e0e39f2ee0488ae3c0d32ae43ffdacbbf7badbfb3dbb1
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

Utiliser l’implémentation Rust fmt.

```rust
impl fmt :: Display for DecimalError
```

## Paramètres, résultat et utilisation courante

Display écrit uniquement le message. Utilisez format! ou to_string() ; examinez séparément kinds pour un traitement automatique. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
