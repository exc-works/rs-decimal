---
locale: fr
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
translation_provenance: en-US-french-reviewed
api_id: Decimal::float32
---
# Decimal::float32

Convertir directement en IEEE binary32 et indiquer l’exactitude.

```rust
fn float32 (& self) -> (f32 , bool)
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie (f32, bool), représenté par [number, exact]. L’arrondi choisit le pair à mi-chemin. L’indicateur signifie que le décimal est représenté exactement, pas simplement que la conversion a réussi : 0.1 est inexact. Un dépassement renvoie un infini signé avec false ; un sous-dépassement peut renvoyer un zéro signé. La conversion arrondit directement en binary32 sans arrondi intermédiaire en binary64. L’exemple enregistre les bits IEEE pour que JSON ne masque pas une valeur spéciale. print dans le Playground identifie également NaN, les infinis et le zéro négatif avec leurs bits binary64 transportés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
