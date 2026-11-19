---
locale: de
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
translation_provenance: en-US-german-reviewed
api_id: "validation"
---
# validation

Regelfehler in Reihenfolge sammeln und übersetzen.

```rust
pub mod validation
```

## Parameter, Rückgabe und übliche Verwendung

Rust benötigt das im Browserbuild aktivierte Feature validation. Das Modul vereint exakte Regelauswertung und eigene Nachrichtenvorlagen ohne Framework oder globale Registry. Verwenden Sie im Browser den validation-Namensraum. Feldnamen sind Text des Aufrufers und werden bei Übersetzung wörtlich eingesetzt. Verwenden Sie required explizit, wenn fehlende Eingaben scheitern müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
