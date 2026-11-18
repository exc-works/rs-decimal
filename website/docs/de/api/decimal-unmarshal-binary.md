---
locale: de
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Eine Dezimalzahl aus einem Go-kompatiblen Binärskalar ersetzen.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Die ersten vier Bytes codieren eine Big-Endian-Skala, gefolgt von Gob-Header 2 (positiv) oder 3 (negativ) und Absolutwert. Erfolgreiches Decodieren erhält die codierte Skala. Leere Eingabe erzeugt initialisierte Null, anders als die nicht initialisierte Standardnull. Ein vier Byte langes Präfix ohne Gob-Payload wird ebenfalls als initialisierte Null mit dieser Skala akzeptiert. Kurze Präfixe, ungültige Versionen und Skalen über 131072 liefern Unmarshal-Kernfehler. Fehler erhalten den Empfänger.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="de" />

## Grenzen im Browser

Die Eingabe muss Uint8Array und kind eine Ganzzahl zwischen 0 und 255 sein. Eingaben sind vor dem Kopieren nach Wasm auf 65536 Bytes begrenzt. Rust prüft binäre Skalen bis 1000 und tatsächliche Koeffizientengrößen bis 16384 Bits. BSON-Zeichenfolgen folgen der Textgrenze von 8192 Bytes; Decimal128-Exponentenexpansion wird ebenfalls [auf Ressourcen geprüft](../resource-policy). Diese Grenzen können enger als die der nativen Bibliothek sein. Prüfungen erfolgen vor Empfängeränderungen; Kern- und Ressourcenfehler bleiben getrennt, verschobene Handles sind nicht wiederverwendbar.

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
