---
locale: fr
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
translation_provenance: en-US-french-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

Grouper les chiffres entiers et choisir le séparateur décimal.

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## Paramètres, résultat et utilisation courante

thousands : char ; decimal : char. Renvoie String/string avec des groupes de trois chiffres entiers et toutes les décimales stockées. Chaque argument JavaScript doit être exactement un scalaire Unicode, y compris un caractère supplémentaire ; les substituts isolés et les chaînes de plusieurs caractères sont rejetés. NUL pour thousands désactive le regroupement ; NUL pour decimal choisit un point. Les séparateurs sont des caractères littéraux, pas des noms de paramètres régionaux.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
