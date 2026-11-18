---
locale: de
source_revision: 0bdc18aaeece0f82160084e0821c2b46ff15a76d9ea8f28bd42767036c596312
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Clone erhält den vollständigen Wert; ausgeliehene ScanValue-Slices verweisen weiterhin auf die ursprünglichen Daten. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
