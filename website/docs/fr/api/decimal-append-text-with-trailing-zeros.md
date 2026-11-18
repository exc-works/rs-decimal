---
locale: fr
source_revision: 9028fbd5554cd396883d5b8bb4b06efa13e22d1b025f5ba855a44ab02e548531
translation_provenance: en-US-french-reviewed
api_id: Decimal::append_text_with_trailing_zeros
---
# Decimal::append_text_with_trailing_zeros

Ajouter des octets ASCII décimaux à un tampon mutable existant.

```rust
fn append_text_with_trailing_zeros (& self , dst : & mut Vec < u8 >)
```

## Paramètres, résultat et utilisation courante

`dst: &mut Vec<u8>`, représenté par un objet enveloppe ByteBuffer. Rust renvoie () ; JavaScript renvoie undefined. Les octets existants sont conservés et la destination est modifiée sur place. Tous les zéros fractionnaires stockés sont conservés. Decimal et le tampon restent utilisables. ByteBuffer.new copie le Uint8Array ; to_uint8_array renvoie une copie, pas une vue de la mémoire Wasm. Une erreur de ressources survient avant la modification et laisse le tampon inchangé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="fr" />

## Limites du navigateur

La [politique de ressources](../resource-policy) limite la largeur du format à 65536 octets et la précision à 1000 ; le format fixe contrôle également la croissance du coefficient. Les tampons sont limités à 65536 octets et l’ajout utilise une estimation conservatrice de la taille, qui peut rejeter une opération tenant exactement. Les erreurs de ressources ne modifient ni les entrées ni les tampons. print tronque séparément les textes longs à l’affichage ; examinez length ou affichez des tranches plus courtes pour vérifier un résultat complet.

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
