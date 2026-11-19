---
locale: fr
source_revision: cc1566925b96baa324fb64a51eba19d80177bb315f68b0f6a9ee1d05c6da9807
translation_provenance: en-US-french-reviewed
api_id: Decimal::exp
---
# Decimal::exp

Calculer l’exponentielle e^x. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. Sans paramètre. La précision de sortie par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. Le cœur accepte toutes les entrées réelles. exp(0) est la valeur 1 initialisée ; un argument suffisamment négatif peut être correctement arrondi à un zéro initialisé à la précision demandée. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
