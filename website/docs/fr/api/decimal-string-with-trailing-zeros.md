---
locale: fr
source_revision: 7fbcc33cd9efa5ed33f322356ce37cb91f352f88d4c3bb3845c8245fa3ded94d
translation_provenance: en-US-french-reviewed
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

Formater un texte décimal en notation fixe.

```rust
fn string_with_trailing_zeros (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string sans notation scientifique. Conserve chaque décimale stockée, y compris les zéros finaux. Utilise un point et aucun regroupement. Le zéro négatif n’a pas de représentation distincte. Le zéro par défaut affiche 0 sans être initialisé ; le formatage emprunte l’entrée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
