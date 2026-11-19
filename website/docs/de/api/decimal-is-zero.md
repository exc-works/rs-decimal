---
locale: de
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

Vor einer wertabhängigen Operation auf numerische Null prüfen.

```rust
fn is_zero (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean und auch für den nicht initialisierten Standardwert true. Verwenden Sie is_nil(), wenn die Initialisierung relevant ist.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
