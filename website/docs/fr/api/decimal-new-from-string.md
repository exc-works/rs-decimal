---
locale: fr
source_revision: 3ef0ab66b1050d81978ffe9504c298d164d62d052b64bfd1ad02ddce4a039f36
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

Analyser un texte décimal exact sans passer par un flottant binaire. C’est le constructeur habituel pour les montants décimaux saisis par l’utilisateur.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`text: &str` → `string` JavaScript ; Rust renvoie `Result<Decimal, DecimalError>`, tandis que le bridge renvoie un Decimal ou lève une erreur avec les mêmes catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="fr" />

## Remarques et limites

Les espaces aux extrémités sont supprimés ; l’entrée peut contenir un signe et un exposant. Des chiffres sont nécessaires de part et d’autre du point décimal. Le séparateur reste `.`, quelle que soit la langue de l’interface. Les zéros finaux des valeurs non nulles sont conservés, mais `"0.00"` est normalisé en zéro initialisé d’échelle zéro. Une syntaxe incorrecte produit `InvalidFormat` ; l’expansion des entrées peut atteindre la limite de ressources distincte du playground.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
