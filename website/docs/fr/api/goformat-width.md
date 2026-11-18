---
locale: fr
source_revision: 0ad883afdae2ec6d62af55006e383eb095f5c7928dcdbf6c2f7baf446a49d945
translation_provenance: en-US-french-reviewed
api_id: GoFormat.width
---
# GoFormat.width

Définir le champ de format width.

```rust
width: Option < usize >
```

## Paramètres, résultat et utilisation courante

La largeur minimale inclut le signe et le point décimal ; le remplissage ne tronque pas un texte plus long. None/null désactive la largeur minimale. Le formatage du cœur compte les octets de sortie.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
