---
locale: fr
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
translation_provenance: en-US-french-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

Compter les bits de la valeur absolue du coefficient non mis à l’échelle.

```rust
fn bit_len (& self) -> usize
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie usize, le bridge renvoie number. L’échelle n’est pas appliquée et le bit de signe n’est pas compté : -1.20 a pour coefficient -120 et pour longueur en bits 7. Zéro, y compris le zéro par défaut, renvoie 0. Ce n’est pas l’indice du bit de poids fort à 1.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
