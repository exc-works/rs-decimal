---
locale: fr
source_revision: 07fd0624795c1d5a39d29225db8108c9026886d41981fb4ff89e2fb3be01274d
translation_provenance: en-US-french-reviewed
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

Calculer l’exponentielle e^x. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. prec : isize non négatif, représenté par un number entier, choisit le nombre de décimales ; une précision négative renvoie InvalidPrecision avant les contrôles de domaine. Le cœur accepte toutes les entrées réelles. exp(0) est la valeur 1 initialisée ; un argument suffisamment négatif peut être correctement arrondi à un zéro initialisé à la précision demandée. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
