---
locale: de
source_revision: 50a6a7b47a0234b339cf6d517b9928ec7d1775457e7eacbe40da86286996212a
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

Einen BSON-Skalar in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

kind: u8/number, data: &[u8]/Uint8Array; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt bson. Unterstützt double (1), string (2), null (10), int32 (16), int64 (18) und Decimal128 (19); numerische Payloads verwenden BSON-Little-Endian-Bytes. Übergeben Sie int64 als rohe Bytes, um Genauigkeitsverlust durch JS number zu vermeiden. BSON null setzt den Empfänger auf nicht initialisierte Null zurück, anders als unmarshal_json(null), das ihn unverändert lässt. Erfolgreiche nullable-Decodierung einer Eingabe außer null setzt valid=true. Nicht unterstützte Arten, abgeschnittene Daten und nicht endliche Zahlen scheitern ohne Teiländerung. Kompatibilitätsdetails: Zeichenfolgendecodierung überspringt das deklarierte letzte Byte ohne NUL-Prüfung; Bytes hinter der benötigten Payload werden ignoriert. Decimal128 wird vom Rust-Kern decodiert und behält unterstützte Dezimalgenauigkeit; marshal_bson_value gibt weiterhin eine Zeichenfolge aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
