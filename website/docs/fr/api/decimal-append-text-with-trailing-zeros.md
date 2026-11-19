---
locale: fr
source_revision: 8f3a1f2c3a3bbab8fcd006696e4bbfc5801d795ba8c28546dfa164fc41913e69
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

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
