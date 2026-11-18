---
locale: fr
source_revision: 93c9a590df79625539c288c9f6a42d27aa1dba571b28363b98f9ec4642104e16
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Utiliser l’implémentation Rust Ord.

```rust
impl Ord for ErrorKind
```

## Paramètres, résultat et utilisation courante

Ordonne ErrorKind selon l’ordre de déclaration Rust, pas selon la gravité ou l’ordre alphabétique des noms. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
