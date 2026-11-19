---
locale: fr
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
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

## Méthodes associées

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
