---
locale: fr
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
translation_provenance: en-US-french-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

Analyser un texte décimal de confiance lorsque des données incorrectes constituent une erreur de programmation.

```rust
fn must_from_string (text : & str) -> Self
```

## Paramètres, résultat et remarques

text : &str/string ; renvoie Decimal. Une entrée valide produit le même état que new_from_string. Une syntaxe incorrecte déclenche un panic avec DecimalError en Rust ; le Wasm compilé déclenche un trap plutôt qu’une erreur de bibliothèque récupérable. Préférez new_from_string pour les entrées utilisateur. Un trap non intercepté termine l’exécution courante ; démarrez une nouvelle exécution avant de réutiliser Rust. Les contrôles de paramètres et de ressources rejettent toujours les entrées concernées avant l’appel à Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="fr" />

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
