---
locale: fr
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
translation_provenance: en-US-french-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

Ajouter des octets ASCII décimaux à un tampon mutable existant.

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## Paramètres, résultat et utilisation courante

`dst: &mut Vec<u8>`, représenté par un objet enveloppe ByteBuffer. Rust renvoie () ; JavaScript renvoie undefined. Les octets existants sont conservés et la destination est modifiée sur place. Les zéros fractionnaires finaux sont supprimés. Decimal et le tampon restent utilisables. ByteBuffer.new copie le Uint8Array ; to_uint8_array renvoie une copie, pas une vue de la mémoire Wasm. Une erreur de ressources survient avant la modification et laisse le tampon inchangé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="fr" />

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
