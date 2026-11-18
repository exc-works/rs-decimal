---
locale: fr
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
translation_provenance: en-US-french-reviewed
api_id: "Decimal::value"
---
# Decimal::value

Obtenir la valeur SQL canonique.

```rust
fn value (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Même un Decimal non initialisé renvoie "0", qui n’est pas NULL ; utilisez NullDecimal si SQL NULL est nécessaire. Supprime les zéros fractionnaires finaux sans changer l’échelle ni l’initialisation. Le texte renvoyé est une valeur scalaire, pas un fragment de requête SQL entre guillemets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
