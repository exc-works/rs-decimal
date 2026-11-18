---
locale: fr
source_revision: 0d692c90930959d30189d9870456cce3ed78a046b51b42c8004981e031486270
translation_provenance: en-US-french-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

Appliquer un spécificateur de format compatible Go et ses options.

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## Paramètres, résultat et utilisation courante

f : GoFormat ; renvoie Result<String, DecimalError>/string. Accepte un seul spécificateur, pas un motif printf tel que %.2f. Les spécificateurs pris en charge sont v/s/q/d/f/F/e/E/g/G/b. L’arrondi est au pair. Un spécificateur non pris en charge ou d appliqué à une valeur non entière produit un marqueur %! sous forme de texte, pas une exception. Une précision fixe dépassant la plage signée native renvoie InvalidPrecision ; les limites du navigateur sur la largeur, la précision et les allocations sont distinctes. Les champs JavaScript omis utilisent les valeurs par défaut de Rust ; les champs inconnus et les types de champs incorrects sont rejetés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
