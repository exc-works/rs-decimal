---
locale: de
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
translation_provenance: en-US-german-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

Regelfehler in Reihenfolge sammeln und übersetzen.

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## Parameter, Rückgabe und übliche Verwendung

Rust benötigt das im Browserbuild aktivierte Feature validation. field: &str/string, value: &Decimal/Decimal, rules: &[ValidationRule]/Array; liefert jeden fehlgeschlagenen ValidationError in Aufruferreihenfolge. Doppelte Regeln können doppelte Fehler erzeugen; es gibt weder Reflection noch Registry oder vorzeitigen Abbruch. Leere Regeln liefern ein leeres Array. Alle Eingaben werden ausgeliehen. Feldnamen sind Text des Aufrufers und werden bei Übersetzung wörtlich eingesetzt. Verwenden Sie required explizit, wenn fehlende Eingaben scheitern müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
