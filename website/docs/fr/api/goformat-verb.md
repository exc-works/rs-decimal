---
locale: fr
source_revision: 349cbd7b8c032b6b9b799b90d2d1981de5ba6641cd94e6d5d2f6cdc9c436f5e2
translation_provenance: en-US-french-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

Définir le champ de format verb.

```rust
verb: char
```

## Paramètres, résultat et utilisation courante

Choisir un spécificateur de format constitué d’un seul scalaire Unicode. Par défaut, v utilise une notation fixe compacte ; e/E utilisent la notation scientifique, q ajoute des guillemets et b affiche le coefficient binaire suivi de p-scale, et non un flottant IEEE.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
