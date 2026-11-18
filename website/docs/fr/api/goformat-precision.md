---
locale: fr
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
translation_provenance: en-US-french-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

Définir le champ de format precision.

```rust
precision: Option < usize >
```

## Paramètres, résultat et utilisation courante

La précision dépend du spécificateur : f/F comptent les décimales, e/E les positions après le premier chiffre, g/G les chiffres significatifs (zéro est traité comme un), et d le nombre minimal de chiffres entiers. None/null conserve les règles par défaut du spécificateur ; f conserve l’échelle, e utilise six décimales.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
