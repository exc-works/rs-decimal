---
locale: fr
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
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

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
