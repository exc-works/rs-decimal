---
locale: fr
source_revision: c212a88589b656fbaa3ad6d1c770f0018ed83f06532037fe1e09556d05cb61ea
translation_provenance: en-US-french-reviewed
api_id: Decimal::log2_with_prec
---
# Decimal::log2_with_prec

Calculer le logarithme en base deux. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. prec : isize non négatif, représenté par un number entier, choisit le nombre de décimales ; une précision négative renvoie InvalidPrecision avant les contrôles de domaine. L’entrée doit être positive ; zéro et les entrées négatives renvoient InvalidLog. Les puissances exactes de la base ont des logarithmes entiers exacts, conservés à l’échelle de sortie. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn log2_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2-with-prec" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
