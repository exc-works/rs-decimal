---
locale: fr
source_revision: f3a534483ccc1e91ef144a2c4109aaf3dd71447ded56bb396f1a854b09445c8e
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

## Limites du navigateur

Cette famille impose des [limites de ressources](../resource-policy) supplémentaires : précision de sortie de 128 au maximum, échelle d’entrée de 256 au maximum et coefficient d’entrée de 4096 bits au maximum. Le calcul ordinaire de exp exige aussi une entrée de valeur absolue au plus 64 ; les entrées négatives dont le cœur prouve l’arrondi direct à zéro conservent ce chemin rapide. Une grande échelle d’entrée peut conduire une méthode sans argument à choisir une précision par défaut supérieure à 128 ; utilisez with_prec pour choisir explicitement une précision de sortie autorisée. Les erreurs de ressources diffèrent de NoConvergence du cœur. Stop interrompt l’exécution ; la suivante crée une nouvelle instance Wasm.

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
