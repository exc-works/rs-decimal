---
locale: fr
source_revision: c41b021290595dc4c30480f34c9ca6fcd54ae6ed90ebcef6b793849dbfd2318c
translation_provenance: en-US-french-reviewed
api_id: Decimal::float64
---
# Decimal::float64

Convertir directement en IEEE binary64 et indiquer l’exactitude.

```rust
fn float64 (& self) -> (f64 , bool)
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie (f64, bool), représenté par [number, exact]. L’arrondi choisit le pair à mi-chemin. L’indicateur signifie que le décimal est représenté exactement, pas simplement que la conversion a réussi : 0.1 est inexact. Un dépassement renvoie un infini signé avec false ; un sous-dépassement peut renvoyer un zéro signé. L’exemple enregistre les bits IEEE pour que JSON ne masque pas une valeur spéciale. print dans le Playground identifie également NaN, les infinis et le zéro négatif avec leurs bits binary64 transportés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float64" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
