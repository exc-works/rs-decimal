---
locale: fr
source_revision: ea81b480a27af3babe08d7b00fe374cc7cd1626376f6bcd3383174c9fcd41789
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

Analyser un texte décimal exact sans passer par un flottant binaire. C’est le constructeur habituel pour les montants décimaux saisis par l’utilisateur.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`text: &str` → `string` JavaScript ; Rust renvoie `Result<Decimal, DecimalError>`, tandis que l’API JavaScript renvoie un Decimal ou lève une erreur avec les mêmes catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="fr" />

## Conseils d’utilisation

Les espaces aux extrémités sont supprimés ; l’entrée peut contenir un signe et un exposant. Des chiffres sont nécessaires de part et d’autre du point décimal. Le séparateur reste `.`, quelle que soit la langue de l’interface. Les zéros finaux des valeurs non nulles sont conservés, mais `"0.00"` est normalisé en zéro initialisé d’échelle zéro. Une syntaxe incorrecte produit `InvalidFormat`.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
