---
locale: fr
source_revision: cd679689311c552a10e0f46bebe2c6d4b3b7c9ae648dc393862458b144cd8b20
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

## Limites du navigateur

Cette famille impose des [limites de ressources](../resource-policy) supplémentaires : précision de sortie de 128 au maximum, échelle d’entrée de 256 au maximum et coefficient d’entrée de 4096 bits au maximum. Le calcul ordinaire de exp exige aussi une entrée de valeur absolue au plus 64 ; les entrées négatives dont le cœur prouve l’arrondi direct à zéro conservent ce chemin rapide. Une grande échelle d’entrée peut conduire une méthode sans argument à choisir une précision par défaut supérieure à 128 ; utilisez with_prec pour choisir explicitement une précision de sortie autorisée. Les erreurs de ressources diffèrent de NoConvergence du cœur. Stop interrompt l’exécution ; la suivante crée une nouvelle instance Wasm.

## Méthodes associées

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
