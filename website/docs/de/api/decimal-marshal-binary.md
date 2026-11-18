---
locale: de
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Das Go-kompatible precision/Gob-Übertragungsformat codieren.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Result<Vec<u8>, DecimalError>`/Uint8Array. Initialisierte Werte entfernen nachgestellte Nachkommanullen, schreiben dann eine vier Byte lange Big-Endian-Skala, das big.Int-Gob-Versions-/Vorzeichenbyte und den Big-Endian-Absolutwert. Dies ist eine dezimale Skalar-Payload, kein IEEE-Gleitkommawert. Nicht initialisierte Null ergibt leere Bytes; initialisierte Null ergibt [0,0,0,0,2]. Die zurückgegebene Kopie kann ohne Änderung des Decimal verändert werden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="de" />

## Grenzen im Browser

Die Eingabe muss Uint8Array und kind eine Ganzzahl zwischen 0 und 255 sein. Eingaben sind vor dem Kopieren nach Wasm auf 65536 Bytes begrenzt. Rust prüft binäre Skalen bis 1000 und tatsächliche Koeffizientengrößen bis 16384 Bits. BSON-Zeichenfolgen folgen der Textgrenze von 8192 Bytes; Decimal128-Exponentenexpansion wird ebenfalls [auf Ressourcen geprüft](../resource-policy). Diese Grenzen können enger als die der nativen Bibliothek sein. Prüfungen erfolgen vor Empfängeränderungen; Kern- und Ressourcenfehler bleiben getrennt, verschobene Handles sind nicht wiederverwendbar.

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
