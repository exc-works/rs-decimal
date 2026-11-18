---
locale: fr
source_revision: 5323ce455eba922d5a5c86c41373b1d3638bbe014511b21ca1e6f036392adab6
translation_provenance: en-US-french-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

Définir le champ de format alternate.

```rust
alternate: bool
```

## Paramètres, résultat et utilisation courante

Conserver un point décimal pour f/F/e/E et les zéros finaux du format général pour g/G. Cela ne signifie pas un changement de base numérique. Valeur par défaut : false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
