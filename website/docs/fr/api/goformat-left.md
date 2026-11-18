---
locale: fr
source_revision: 1bcdb4a809c3db807556fea8772e6afac8daed8fef0faa997fbb848ae1a34a13
translation_provenance: en-US-french-reviewed
api_id: GoFormat.left
---
# GoFormat.left

Définir le champ de format left.

```rust
left: bool
```

## Paramètres, résultat et utilisation courante

Compléter à droite avec des espaces. L’alignement à gauche est prioritaire sur le remplissage numérique par des zéros. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
