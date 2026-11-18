---
locale: fr
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: en-US-french-reviewed
api_id: GoFormat
---
# GoFormat

Décrire une opération de formatage par un objet de champs.

```rust
pub struct GoFormat
```

## Paramètres, résultat et utilisation courante

Les champs correspondent à la structure Rust : verb, width, precision, left, plus, space, zero et alternate. GoFormat.default() dans le bridge lit les valeurs par défaut de Rust et renvoie un nouvel objet mutable ; sa modification ne définit pas un format global. width/precision utilisent null pour None et des number entiers non négatifs dans la plage wasm32 pour Some. La syntaxe des traits Rust reste uniquement native.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
