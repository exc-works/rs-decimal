---
locale: fr
source_revision: ea6f97f51071f45a8c9bf2f4df22faacd07f9c9b025ed9b3ebf43ae9fab4e095
translation_provenance: en-US-french-reviewed
api_id: Decimal::log10
---
# Decimal::log10

Calculer le logarithme en base dix. Sans paramètre. La précision de sortie par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. L’entrée doit être positive ; zéro et les entrées négatives renvoient InvalidLog. Les puissances exactes de la base ont des logarithmes entiers exacts, conservés à l’échelle de sortie. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn log10 (& self) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
