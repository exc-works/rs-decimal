---
locale: fr
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
translation_provenance: en-US-french-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

Valider un discriminant d’arrondi brut avec TryFrom.

```rust
type Error = DecimalError
```

## Utilisation courante et points à retenir

value : i32 ; renvoie `Result<RoundingMode, DecimalError>`. Les codes 0–6 utilisent la correspondance figée des modes, dont 5=HalfEven. Les codes inconnus renvoient InvalidArgument sans panique. Le type associé Error est DecimalError. Cette opération délègue à from_raw ; le navigateur utilise RoundingMode.from_raw(number), avec des contrôles stricts du paramètre i32.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
