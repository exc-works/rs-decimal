---
locale: de
source_revision: b2646b52b58adc30058be6fedbac290221f3b2b947946b06b25fe8783f697afc
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

json-Eingabe in einen vorhandenen Decimal decodieren.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Rust benötigt json. Übergeben Sie ursprüngliche UTF-8-JSON-Bytes: Zahlen werden ohne Gleitkommaumweg validiert, Escapes in Zeichenfolgen vor exaktem Dezimalparsen aufgelöst. Wandeln Sie Zahlentoken nicht zuerst mit JSON.parse in JS-Zahlen um. JSON-Leerraum ist ausschließlich space/tab/CR/LF. JSON null lässt den gesamten vorhandenen Decimal einschließlich Initialisierung unverändert. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
