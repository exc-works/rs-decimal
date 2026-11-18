---
locale: fr
source_revision: 6ee970e3e2507cdd225eb4c314175b5a95b9aa45ac2e64ed4be5fded0c11b425
translation_provenance: en-US-french-reviewed
api_id: GoFormat.space
---
# GoFormat.space

Définir le champ de format space.

```rust
space: bool
```

## Paramètres, résultat et utilisation courante

Ajouter un espace initial aux sorties numériques non négatives lorsque plus vaut false. Les valeurs négatives conservent leur signe. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
