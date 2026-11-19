---
locale: de
source_revision: 6fa89e7d9fd5a43bce6d83be2495a38047592a77e9611dcd781da000b62e4862
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

text-Eingabe in einen vorhandenen Decimal decodieren.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Parst Dezimaltext einschließlich entferntem äußerem Leerraum und Exponentensyntax. Leerer Text und das Literal null sind ungültiger Dezimaltext; verwenden Sie nullable-Decodierung, wenn Leere Abwesenheit bedeutet. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
