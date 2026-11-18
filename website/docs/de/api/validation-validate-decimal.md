---
locale: de
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
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

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
