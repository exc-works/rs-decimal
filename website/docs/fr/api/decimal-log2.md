---
locale: fr
source_revision: a329356af92d18ac765420533258b5c608220d0cf27b5772304a0e1b5b8d5f9e
translation_provenance: en-US-french-reviewed
api_id: Decimal::log2
---
# Decimal::log2

Calculer le logarithme en base deux. Renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception structurée dans le bridge. Sans paramètre. La précision de sortie par défaut est max(nombre de décimales de l’entrée, 20), pas 20 chiffres significatifs. L’entrée doit être positive ; zéro et les entrées négatives renvoient InvalidLog. Les puissances exactes de la base ont des logarithmes entiers exacts, conservés à l’échelle de sortie. Le cœur encadre les résultats par des intervalles et ne les accepte que lorsque l’arrondi est certifié. L’arrondi est au pair, sans paramètre mode ; l’échec de la certification dans les budgets du cœur renvoie NoConvergence plutôt qu’une approximation non vérifiée. Les entrées restent inchangées et les résultats réussis sont initialisés.

```rust
fn log2 (& self) -> Result < Self , DecimalError >
```

## Utilisation courante

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2" locale="fr" />

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
