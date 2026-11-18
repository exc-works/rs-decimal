---
locale: fr
source_revision: b8d0350ab2d3257ebc2822de61360d2033648c92799436b97481a4c0e75947bf
translation_provenance: en-US-french-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

Définir le champ de format plus.

```rust
plus: bool
```

## Paramètres, résultat et utilisation courante

Afficher + pour les sorties numériques non négatives. Prioritaire sur space. S’applique aux spécificateurs numériques, pas à v/s/q ; les valeurs négatives conservent -. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
