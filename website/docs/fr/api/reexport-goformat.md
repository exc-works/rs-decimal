---
locale: fr
source_revision: 4beaa8ccdf2751ba7bce74d134b8de29fabd5c0d82d0d4eebee1575a204be12d
translation_provenance: en-US-french-reviewed
api_id: reexport::GoFormat
---
# reexport::GoFormat

Importer GoFormat depuis la racine du crate.

```rust
pub use formatting::GoFormat as GoFormat
```

## Paramètres, résultat et utilisation courante

Il s’agit de la réexportation publique de la structure Rust, pas d’une autre implémentation. Le navigateur fournit un adaptateur sous forme d’objet de champs et une fonction de création des valeurs par défaut déléguée à Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-reexport-goformat" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
