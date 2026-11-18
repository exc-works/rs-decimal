---
locale: fr
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

Obtenir la valeur SQL canonique.

```rust
fn value (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Les valeurs nullables invalides renvoient None/null, quelles que soient les données internes masquées. Les valeurs valides renvoient le texte décimal canonique, y compris "0" pour un zéro interne non initialisé. Supprime les zéros fractionnaires finaux sans changer l’échelle ni l’initialisation. Le texte renvoyé est une valeur scalaire, pas un fragment de requête SQL entre guillemets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
