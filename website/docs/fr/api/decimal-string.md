---
locale: fr
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
translation_provenance: en-US-french-reviewed
api_id: Decimal::string
---
# Decimal::string

Formater un texte décimal en notation fixe.

```rust
fn string (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string sans notation scientifique. Supprime les zéros fractionnaires finaux superflus ; les zéros entiers restent présents. Utilise un point et aucun regroupement. Le zéro négatif n’a pas de représentation distincte. Le zéro par défaut affiche 0 sans être initialisé ; le formatage emprunte l’entrée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
