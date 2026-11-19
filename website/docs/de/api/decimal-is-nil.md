---
locale: de
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

Prüfen, ob eine Dezimalzahl der nicht initialisierte Standardwert ist.

```rust
fn is_nil (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean. Der Standardwert ist nil, konstruierte Null ist initialisiert; beide sind numerisch gleich. Dies ist nicht das Gültigkeitskennzeichen von NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
