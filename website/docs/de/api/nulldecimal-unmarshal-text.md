---
locale: de
source_revision: 949ffe2e68cd75d5b05be8db37757052323a51952d74093deec642c235953650
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_text"
---
# NullDecimal::unmarshal_text

text-Eingabe in einen vorhandenen NullDecimal decodieren.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Parst Dezimaltext einschließlich entferntem äußerem Leerraum und Exponentensyntax. Leere Eingabe löscht die Gültigkeit und setzt den inneren Decimal zurück; das Literal null ist ungültiger Dezimaltext. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Setzt valid=true. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-text" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
