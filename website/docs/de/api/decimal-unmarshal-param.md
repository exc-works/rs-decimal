---
locale: de
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

param-Eingabe in einen vorhandenen Decimal decodieren.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

param: &str/string; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Parst Dezimaltext einschließlich entferntem äußerem Leerraum und Exponentensyntax. Leerer Text und das Literal null sind ungültiger Dezimaltext; verwenden Sie nullable-Decodierung, wenn Leere Abwesenheit bedeutet. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
