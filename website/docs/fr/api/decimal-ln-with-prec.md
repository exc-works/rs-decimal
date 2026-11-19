---
locale: fr
source_revision: 14996308295f3d9761be44a618304820607fea78ed2b85488ce4a81975821f38
translation_provenance: en-US-french-reviewed
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

Calculer le logarithme naturel. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. prec : isize non négatif, représenté par un number entier, choisit le nombre de décimales ; une précision négative renvoie InvalidPrecision avant les contrôles de domaine. L’entrée doit être positive ; zéro et les entrées négatives renvoient InvalidLog. ln(1) vaut exactement zéro. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
